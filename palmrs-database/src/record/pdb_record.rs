//! Palm database record headers
//!
//! This module contains the [`PdbRecordHeader`] enum, representing either a "record" or a
//! "resource" entry within a Palm database file. See the documentation for that enum for more
//! details.

use core::{fmt::Debug, str};
use std::io::{self, Cursor, Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::{header::DatabaseHeader, record::DatabaseRecord};

/// Generic record header type helper trait
pub trait PdbRecordHeaderTrait {
	fn record_struct_len(&self) -> usize;
	fn next_entry_data_offset(&self) -> usize;
}

/// "Resource" header type
impl PdbRecordHeaderTrait for PdbRecordHeader {
	fn record_struct_len(&self) -> usize {
		match self {
			PdbRecordHeader::Record { .. } => 8,
			PdbRecordHeader::Resource { .. } => 10,
		}
	}
	fn next_entry_data_offset(&self) -> usize {
		match self {
			PdbRecordHeader::Record { .. } => 0,
			PdbRecordHeader::Resource { .. } => 6,
		}
	}
}

/// Generic Palm database record header
///
/// This type can represent either a "record" or a "resource" within a Palm OS database:
///
/// - Records are potentially-mutable entries that are used in PDB databases to represent content;
/// - Resources are generally-immutable data entries that are used, as the name would suggest, for
///   application resources, and are found within PRC files.
///
/// Which type to use to decode a given record header should be determined by the lowest bit in the
/// `attributes` field of the [`DatabaseHeader`] for this record's containing database - if that
/// bit is cleared, "records" are used; if it is set, "resources" are used.
///
/// These two types are presented as an `enum` for the sake of flexibility, and to allow sharing
/// encoding/decoding code between the two available record header types.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PdbRecordHeader {
	Record {
		attributes: u8,
		unique_id: u32,
		data_offset: u32,
		data_len: Option<u32>,
	},

	Resource {
		name: [u8; 4],
		record_id: u16,
		data_offset: u32,
		data_len: Option<u32>,
	},
}

impl DatabaseRecord for PdbRecordHeader {
	fn struct_len(&self) -> usize {
		<Self as PdbRecordHeaderTrait>::record_struct_len(self)
	}

	fn from_bytes(hdr: &DatabaseHeader, data: &[u8], pos: usize) -> Result<Self, io::Error> {
		let mut rdr = Cursor::new(&data);
		rdr.seek(SeekFrom::Start(pos as u64))?;

		let mut this = if hdr.attributes & (1 << 0) == 0 {
			// Type bit clear: construct "records"

			let data_offset = rdr.read_u32::<BigEndian>()?;
			let attributes = rdr.read_u8()?;
			let unique_id = rdr.read_u24::<BigEndian>()?;

			Self::Record {
				attributes,
				unique_id,
				data_offset,
				data_len: None,
			}
		} else {
			// Type bit set: construct "resources"

			let name = {
				let mut buf = [0u8; 4];
				rdr.read_exact(&mut buf)?;
				buf
			};

			let record_id = rdr.read_u16::<BigEndian>()?;
			let data_offset = rdr.read_u32::<BigEndian>()?;

			Self::Resource {
				name,
				record_id,
				data_offset,
				data_len: None,
			}
		};

		let actual_len = {
			let data_offset = match this {
				Self::Record { data_offset, .. } => data_offset,
				Self::Resource { data_offset, .. } => data_offset,
			};

			let position = rdr.stream_position()?;
			let data_len = match rdr.seek(SeekFrom::Current(this.next_entry_data_offset() as i64)) {
				Ok(_) => match rdr.read_u32::<BigEndian>() {
					Ok(next_offset) => {
						if next_offset > data_offset {
							next_offset - data_offset
						} else {
							(data.len() as u32) - data_offset
						}
					}

					Err(_) => (data.len() as u32) - data_offset,
				},

				Err(_) => (data.len() as u32) - data_offset,
			};

			rdr.seek(SeekFrom::Start(position))?;
			data_len
		};

		match this {
			Self::Record {
				ref mut data_len, ..
			} => data_len.replace(actual_len),
			Self::Resource {
				ref mut data_len, ..
			} => data_len.replace(actual_len),
		};

		Ok(this)
	}

	fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
		let mut buf = Cursor::new(Vec::with_capacity(self.struct_len()));

		match self {
			Self::Record {
				data_offset,
				attributes,
				unique_id,
				..
			} => {
				buf.write_u32::<BigEndian>(*data_offset)?;
				buf.write_u8(*attributes)?;
				buf.write_u24::<BigEndian>(*unique_id)?;
			}

			Self::Resource {
				name,
				record_id,
				data_offset,
				..
			} => {
				// name
				buf.write_u8(name[0])?;
				buf.write_u8(name[1])?;
				buf.write_u8(name[2])?;
				buf.write_u8(name[3])?;

				// record id & data offset
				buf.write_u16::<BigEndian>(*record_id)?;
				buf.write_u32::<BigEndian>(*data_offset)?;
			}
		}

		Ok(buf.into_inner())
	}

	fn name_str(&self) -> Option<&str> {
		match self {
			Self::Resource { name, .. } => {
				let mut idx = 0;
				while idx < name.len() {
					if name[idx] == 0u8 {
						break;
					}

					idx += 1;
				}

				str::from_utf8(&name[..idx]).ok()
			}

			_ => None,
		}
	}

	fn attributes(&self) -> Option<u32> {
		match self {
			Self::Record { attributes, .. } => Some(*attributes as u32),
			_ => None,
		}
	}

	fn data_offset(&self) -> u32 {
		match self {
			Self::Record { data_offset, .. } => *data_offset,
			Self::Resource { data_offset, .. } => *data_offset,
		}
	}

	fn data_len(&self) -> Option<u32> {
		match self {
			Self::Record { data_len, .. } => *data_len,
			Self::Resource { data_len, .. } => *data_len,
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{
		header::DATABASE_HEADER_LENGTH,
		record::{pdb_record::PdbRecordHeader, DatabaseRecord},
		DatabaseFormat,
		PalmDatabase,
		PdbDatabase,
		PdbWithCategoriesDatabase,
		PrcDatabase,
	};

	const EXAMPLE_PRC: &'static [u8] = include_bytes!("../../../test-data/hello-v1.prc");
	const EXAMPLE_PDB: &'static [u8] = include_bytes!("../../../test-data/ToDoDB.pdb");
	const MANUAL_PDB: &'static [u8] = include_bytes!("../../../test-data/tWmanual.pdb");

	fn test_db<T: DatabaseFormat>(src_bytes: &'static [u8])
	where
		<T as DatabaseFormat>::RecordHeader: PartialEq<PdbRecordHeader>,
	{
		let database = PalmDatabase::<T>::from_bytes(src_bytes).unwrap();

		eprintln!(
			"Testing records for: {}",
			database.header.name_try_str().unwrap()
		);
		let mut rec_start_offset = DATABASE_HEADER_LENGTH;
		// Test record iteration
		for (_idx, (rec_hdr, rec_data)) in (0..).zip(database.records.iter()) {
			assert_eq!(rec_data.len(), rec_hdr.data_len().unwrap_or(0) as usize);
			assert_eq!(
				rec_hdr,
				&PdbRecordHeader::from_bytes(
					&database.header,
					database.original_data,
					rec_start_offset
				)
				.unwrap()
			);
			rec_start_offset += rec_hdr.struct_len();
		}
	}

	#[test]
	fn test_records_all_db_types() {
		test_db::<PrcDatabase>(&EXAMPLE_PRC);
		test_db::<PdbDatabase>(&MANUAL_PDB);
		test_db::<PdbWithCategoriesDatabase>(&EXAMPLE_PDB);
	}
}
