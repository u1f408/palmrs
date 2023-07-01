//! The common file header used by both the PRC and PDB databases

use core::{
	fmt::{self, Debug, Display},
	str,
};
use std::io::{self, Cursor, Read, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::time::PalmTimestamp;

/// Length, in bytes, of the [`DatabaseHeader`]
pub const DATABASE_HEADER_LENGTH: usize = 78;

/// The common file header used by both the PRC and PDB databases.
///
/// This is located at the start of the database (offset `0`).
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C, packed)]
pub struct DatabaseHeader {
	pub name: [u8; 32],
	pub attributes: u16,
	pub version: u16,
	pub creation_time: PalmTimestamp,
	pub modification_time: PalmTimestamp,
	pub backup_time: PalmTimestamp,
	pub modification_number: u32,
	pub app_info_id: u32,
	pub sort_info_id: u32,
	pub type_code: [u8; 4],
	pub creator_code: [u8; 4],
	pub unique_id_seed: u32,
	pub next_record_list: u32,
	pub record_count: u16,
}

impl DatabaseHeader {
	pub const SIZE: usize = DATABASE_HEADER_LENGTH;

	/// Read the database header from the given byte slice.
	pub fn from_bytes(rdr: &mut Cursor<&[u8]>) -> Result<Self, io::Error> {
		// Read all the data

		let name = {
			let mut buf = [0u8; 32];
			rdr.read_exact(&mut buf)?;
			buf
		};

		let attributes = rdr.read_u16::<BigEndian>()?;
		let version = rdr.read_u16::<BigEndian>()?;
		let creation_time = rdr.read_u32::<BigEndian>()?;
		let modification_time = rdr.read_u32::<BigEndian>()?;
		let backup_time = rdr.read_u32::<BigEndian>()?;
		let modification_number = rdr.read_u32::<BigEndian>()?;
		let app_info_id = rdr.read_u32::<BigEndian>()?;
		let sort_info_id = rdr.read_u32::<BigEndian>()?;

		let type_code = {
			let mut buf = [0u8; 4];
			rdr.read_exact(&mut buf)?;
			buf
		};

		let creator_code = {
			let mut buf = [0u8; 4];
			rdr.read_exact(&mut buf)?;
			buf
		};

		let unique_id_seed = rdr.read_u32::<BigEndian>()?;
		let next_record_list = rdr.read_u32::<BigEndian>()?;
		let record_count = rdr.read_u16::<BigEndian>()?;

		// Populate structure

		let created_header = Self {
			name,
			attributes,
			version,
			creation_time: PalmTimestamp(creation_time),
			modification_time: PalmTimestamp(modification_time),
			backup_time: PalmTimestamp(backup_time),
			modification_number,
			app_info_id,
			sort_info_id,
			type_code,
			creator_code,
			unique_id_seed,
			next_record_list,
			record_count,
		};

		Ok(created_header)
	}

	pub fn to_bytes(self) -> std::io::Result<Vec<u8>> {
		let mut cursor = Cursor::new(vec![0_u8; Self::SIZE]);

		let DatabaseHeader {
			name,
			attributes,
			version,
			creation_time,
			modification_time,
			backup_time,
			modification_number,
			app_info_id,
			sort_info_id,
			type_code,
			creator_code,
			unique_id_seed,
			next_record_list,
			record_count,
		} = self;

		cursor.write(&name)?;
		cursor.write_u16::<BigEndian>(attributes)?;
		cursor.write_u16::<BigEndian>(version)?;
		cursor.write_u32::<BigEndian>(creation_time.0)?;
		cursor.write_u32::<BigEndian>(modification_time.0)?;
		cursor.write_u32::<BigEndian>(backup_time.0)?;
		cursor.write_u32::<BigEndian>(modification_number)?;
		cursor.write_u32::<BigEndian>(app_info_id)?;
		cursor.write_u32::<BigEndian>(sort_info_id)?;
		cursor.write(&type_code)?;
		cursor.write(&creator_code)?;
		cursor.write_u32::<BigEndian>(unique_id_seed)?;
		cursor.write_u32::<BigEndian>(next_record_list)?;
		cursor.write_u16::<BigEndian>(record_count)?;

		Ok(cursor.into_inner())
	}

	/// Return the friendly name of the database as a byte slice, containing only the data bytes
	/// of the name (that is, the null terminating bytes are trimmed).
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

	/// Attempt to convert the friendly name of the database to a [`str`][core::str]
	pub fn name_try_str<'x>(&'x self) -> Result<&'x str, str::Utf8Error> {
		str::from_utf8(self.name_trimmed())
	}

	/// Attempt to convert the database type code to a [`str`][core::str]
	pub fn type_code_try_str<'x>(&'x self) -> Result<&'x str, str::Utf8Error> {
		let mut idx = 0;
		while idx < self.type_code.len() {
			if self.type_code[idx] == 0u8 {
				break;
			}

			idx += 1;
		}

		str::from_utf8(&self.type_code[0..idx])
	}

	/// Attempt to convert the database creator code to a [`str`][core::str]
	pub fn creator_code_try_str<'x>(&'x self) -> Result<&'x str, str::Utf8Error> {
		let mut idx = 0;
		while idx < self.creator_code.len() {
			if self.creator_code[idx] == 0u8 {
				break;
			}

			idx += 1;
		}

		str::from_utf8(&self.creator_code[0..idx])
	}
}

impl Display for DatabaseHeader {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"DatabaseHeader({:?}, created={})",
			self.name_try_str().unwrap_or(""),
			self.creation_time,
		)
	}
}
