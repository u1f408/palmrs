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
	fn struct_len(&self) -> usize;
	fn next_entry_data_offset(&self) -> usize;
}

/// "Record" header type
pub struct RecordHeaderType;
impl PdbRecordHeaderTrait for RecordHeaderType {
	fn struct_len(&self) -> usize {
		8
	}
	fn next_entry_data_offset(&self) -> usize {
		0
	}
}

/// "Resource" header type
pub struct ResourceHeaderType;
impl PdbRecordHeaderTrait for ResourceHeaderType {
	fn struct_len(&self) -> usize {
		10
	}
	fn next_entry_data_offset(&self) -> usize {
		6
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

impl PdbRecordHeader {
	pub fn header_type(&self) -> &dyn PdbRecordHeaderTrait {
		match self {
			Self::Record { .. } => &RecordHeaderType,
			Self::Resource { .. } => &ResourceHeaderType,
		}
	}
}

impl DatabaseRecord for PdbRecordHeader {
	fn struct_len(&self) -> usize {
		self.header_type().struct_len()
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
			let data_len = match rdr.seek(SeekFrom::Current(
				this.header_type().next_entry_data_offset() as i64,
			)) {
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

	fn write_bytes(&self) -> Result<Vec<u8>, io::Error> {
		let mut buf = Cursor::new(Vec::with_capacity(self.header_type().struct_len()));

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
