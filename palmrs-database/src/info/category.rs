// http://preserve.mactech.com/articles/mactech/Vol.21/21.08/PDBFile/index.html reference data

//! Item category record helpers

use core::{fmt::Debug, str};
use std::io::{self, Cursor, Read, Seek, SeekFrom, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

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

// struct // from reference at top of page
// {
//     unsigned short renamedCategories;
//     unsigned char  categoryLabels[16][16];
//     unsigned char  categoryUniqueIDs[16];
//     unsigned char  lastUniqueID;
//     unsigned char  RSVD;
// } 	AppInfoType;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct AppInfoCategories {
	/// The number of categories renamed by the user
	pub renamed_categories: u16,
	pub categories: Vec<ExtraInfoCategory>,
	category_unique_ids: [u8; 16],
	last_unique_id: u8,
	rsvd: u8,

	/// check if it's a data header and shouldn't have categories
	is_data: bool,
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
				continue;
			}

			categories.push(ExtraInfoCategory {
				category_id,
				name,
				renamed: (renamed_flags & (1 << category_id)) != 0,
			});
		}

		let mut category_unique_ids = [0_u8; 16];
		for idx in 0..16 {
			category_unique_ids[idx] = rdr.read_u8()?;
		}

		let last_unique_id = rdr.read_u8()?;
		let rsvd = rdr.read_u8()?;

		Ok(Self {
			renamed_categories: renamed_flags,
			categories,
			category_unique_ids,
			last_unique_id,
			rsvd,
			is_data: true,
		})
	}
}

impl ExtraInfoRecord for AppInfoCategories {
	const SIZE: usize = 2 + 16 * 16 + 16 + 1 + 1;

	fn from_bytes(hdr: &DatabaseHeader, data: &[u8], pos: usize) -> Result<Self, io::Error> {
		Self::from_bytes(hdr, data, pos)
	}

	fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
		if !self.is_data {
			// this is not a data-type record and should not have categories
			return Ok(vec![]);
		}

		let mut cursor = Cursor::new(Vec::new());

		cursor.write_u16::<BigEndian>(self.renamed_categories)?;

		for cat in self.categories.iter() {
			cursor.write(&cat.name)?;
		}
		for _ in self.categories.len()..16 {
			cursor.write(&[0_u8; 16])?;
		}
		cursor.write(&self.category_unique_ids)?;
		cursor.write_u8(self.last_unique_id)?;
		cursor.write_u8(self.rsvd)?;

		Ok(cursor.into_inner())
	}

	fn data_empty(&self) -> bool {
		false
	}

	fn data_item_categories(&self) -> Option<Vec<ExtraInfoCategory>> {
		Some(self.categories.clone())
	}
}
