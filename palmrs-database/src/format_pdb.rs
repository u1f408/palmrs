//! The PDB database format

use core::{
	fmt::{self, Debug, Display},
	str,
};
use std::io::{self, Cursor, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::{header::DatabaseHeader, record::DatabaseRecord, DatabaseFormat};

/// A PDB record header
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PdbRecordHeader {
	pub attributes: u8,
	pub unique_id: u32,
	pub data_offset: u32,
	pub data_len: Option<u32>,
}

impl PdbRecordHeader {}

impl DatabaseRecord for PdbRecordHeader {
	fn struct_len() -> usize {
		8
	}

	fn from_bytes(data: &[u8], pos: usize) -> Result<Self, io::Error> {
		let mut rdr = Cursor::new(&data);
		rdr.seek(SeekFrom::Start(pos as u64))?;

		let data_offset = rdr.read_u32::<BigEndian>()?;
		let attributes = rdr.read_u8()?;
		let unique_id = rdr.read_u24::<BigEndian>()?;

		let data_len = {
			let position = rdr.stream_position()?;
			let data_len = match rdr.seek(SeekFrom::Current(0)) {
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
			attributes,
			unique_id,
			data_offset,
			data_len,
		};

		Ok(created_header)
	}

	fn name_str(&self) -> Option<&str> {
		None
	}

	fn attributes(&self) -> Option<u32> {
		Some(self.attributes as u32)
	}

	fn data_offset(&self) -> u32 {
		self.data_offset
	}

	fn data_len(&self) -> Option<u32> {
		self.data_len
	}
}

impl Display for PdbRecordHeader {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"PdbRecordHeader(attributes={:#X}, offset={})",
			self.attributes, self.data_offset,
		)
	}
}

/// Implementation of [`DatabaseFormat`] for PDB databases
pub struct PdbDatabase;

impl DatabaseFormat for PdbDatabase {
	type RecordHeader = PdbRecordHeader;

	fn is_valid(_data: &[u8], header: &DatabaseHeader) -> bool {
		if header.attributes & (1 << 0) != 0 {
			return false;
		}

		true
	}
}
