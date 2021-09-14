//! Item category record helpers

use core::{fmt::Debug, str};
use std::io::{self, Cursor, Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::{header::DatabaseHeader, info::ExtraInfoRecord};

/// Item record category bitmask
///
/// Using the value of this constant as a bitmask on the `attributes` field of a
/// [`PdbRecordHeader::Record`][crate::record::pdb_record::PdbRecordHeader::Record] will give you
/// the category ID as an integer in the range `0..15`.
#[allow(unused_variables)]
pub const CATEGORY_ATTRIBUTE_MASK: u8 = 0x0F;

/// Representation of an item category
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ExtraInfoCategory {
	/// Category ID
	///
	/// Category IDs are in the range `0..15`, where category ID `0` (zero) is reserved for the
	/// "Unfiled" category. This gives a maximum of 15 user-definable categories.
	pub category_id: u8,

	/// Category name
	///
	/// 16-character null-padded string - the same format used by the [`DatabaseHeader`] `name`
	/// field.
	pub name: [u8; 16],

	/// Has the category been renamed?
	pub renamed: bool,
}

impl ExtraInfoCategory {
	/// Attempt to convert the category name to a [`str`][core::str]
	pub fn name_try_str<'x>(&'x self) -> Result<&'x str, str::Utf8Error> {
		let mut idx = 0;
		while idx < self.name.len() {
			if self.name[idx] == 0u8 {
				break;
			}

			idx += 1;
		}

		str::from_utf8(&self.name[0..idx])
	}
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AppInfoCategories {
	pub categories: Vec<ExtraInfoCategory>,
}

impl AppInfoCategories {
	pub fn from_bytes(hdr: &DatabaseHeader, data: &[u8], pos: usize) -> Result<Self, io::Error> {
		// Do a quick check by type/creator codes for whether we should actually have categories
		if &hdr.type_code[..] != b"DATA" {
			return Ok(Default::default());
		}

		let mut rdr = Cursor::new(&data);
		rdr.seek(SeekFrom::Start(pos as u64))?;

		let mut categories = Vec::new();
		let renamed_flags = rdr.read_u16::<BigEndian>()?;
		for category_id in 0..16 {
			let name = {
				let mut buf = [0u8; 16];
				rdr.read_exact(&mut buf)?;
				buf
			};

			if name == [0u8; 16] {
				break;
			}

			categories.push(ExtraInfoCategory {
				category_id,
				name,
				renamed: (renamed_flags & (1 << category_id)) != 0,
			});
		}

		Ok(Self { categories })
	}
}

impl ExtraInfoRecord for AppInfoCategories {
	fn from_bytes(hdr: &DatabaseHeader, data: &[u8], pos: usize) -> Result<Self, io::Error> {
		Self::from_bytes(hdr, data, pos)
	}

	fn write_bytes(&self) -> Result<Vec<u8>, io::Error> {
		unimplemented!();
	}

	fn data_empty(&self) -> bool {
		false
	}

	fn data_item_categories(&self) -> Option<Vec<ExtraInfoCategory>> {
		Some(self.categories.clone())
	}
}
