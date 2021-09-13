//! The PRC (Palm Resource Code) database format

use core::{
	fmt::{self, Debug, Display},
	str,
};
use std::io::{self, Cursor, Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::{header::DatabaseHeader, record::DatabaseRecord, DatabaseFormat};

/// A PRC record header
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PrcRecordHeader {
	pub name: [u8; 4],
	pub record_id: u16,
	pub data_offset: u32,
	pub data_len: Option<u32>,
}

impl PrcRecordHeader {
	/// Return the name of the record as a trimmed byte slice
	pub fn name_trimmed<'x>(&'x self) -> &'x [u8] {
		let mut idx = 0;
		while idx < self.name.len() {
			if self.name[idx] == 0u8 {
				break;
			}

			idx += 1;
		}

		&self.name[..idx]
	}

	/// Attempt to convert the record name to a [`str`][core::str]
	pub fn name_try_str<'x>(&'x self) -> Result<&'x str, str::Utf8Error> {
		str::from_utf8(self.name_trimmed())
	}
}

impl DatabaseRecord for PrcRecordHeader {
	fn struct_len() -> usize {
		10
	}

	fn from_bytes(data: &[u8], pos: usize) -> Result<Self, io::Error> {
		let mut rdr = Cursor::new(&data);
		rdr.seek(SeekFrom::Start(pos as u64))?;

		let name = {
			let mut buf = [0u8; 4];
			rdr.read_exact(&mut buf)?;
			buf
		};

		let record_id = rdr.read_u16::<BigEndian>()?;
		let data_offset = rdr.read_u32::<BigEndian>()?;

		let data_len = {
			let position = rdr.stream_position()?;
			let data_len = match rdr.seek(SeekFrom::Current(6)) {
				Ok(_) => match rdr.read_u32::<BigEndian>() {
					Ok(next_offset) => {
						if next_offset > data_offset {
							Some(next_offset - data_offset)
						} else {
							Some((data.len() as u32) - data_offset)
						}
					}

					Err(_) => Some((data.len() as u32) - data_offset),
				},

				Err(_) => Some((data.len() as u32) - data_offset),
			};

			rdr.seek(SeekFrom::Start(position))?;
			data_len
		};

		let created_header = Self {
			name,
			record_id,
			data_offset,
			data_len,
		};

		Ok(created_header)
	}

	fn name_str(&self) -> Option<&str> {
		self.name_try_str().ok()
	}

	fn data_offset(&self) -> u32 {
		self.data_offset
	}

	fn data_len(&self) -> Option<u32> {
		self.data_len
	}
}

impl Display for PrcRecordHeader {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"PrcRecordHeader({:?}, offset={})",
			self.name_try_str().unwrap_or(""),
			self.data_offset,
		)
	}
}

/// Implementation of [`DatabaseFormat`] for PRC databases
pub struct PrcDatabase;

impl DatabaseFormat for PrcDatabase {
	type RecordHeader = PrcRecordHeader;

	fn is_valid(_data: &[u8], header: &DatabaseHeader) -> bool {
		if header.attributes & (1 << 0) == 0 {
			return false;
		}

		true
	}
}
