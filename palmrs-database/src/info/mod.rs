//! App info & sort info parsers and helpers

use core::fmt::Debug;
use std::io;

use crate::header::DatabaseHeader;

pub mod category;

/// Helper trait for decoding & encoding "extra data" records (app info / sort info)
pub trait ExtraInfoRecord: Sized + Debug {
	/// Size in bytes (packed) which the ExtraInfoRecord occupies in the pdb/prc
	const SIZE: usize;

	/// Read the record header from the given byte array
	fn from_bytes(hdr: &DatabaseHeader, data: &[u8], pos: usize) -> Result<Self, io::Error>;

	/// Write the record header to a new `Vec<u8>`
	fn to_bytes(&self) -> Result<Vec<u8>, io::Error>;

	/// Whether this ExtraInfoRecord contains no data
	fn data_empty(&self) -> bool;

	/// If the record contains a list of item categories, return those categories
	fn data_item_categories(&self) -> Option<Vec<category::ExtraInfoCategory>> {
		None
	}
}

/// Null implementation of [`ExtraInfoRecord`]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NullExtraInfo;
impl ExtraInfoRecord for NullExtraInfo {
	const SIZE: usize = 0;

	fn from_bytes(_hdr: &DatabaseHeader, _data: &[u8], _pos: usize) -> Result<Self, io::Error> {
		Ok(Self)
	}

	fn to_bytes(&self) -> Result<Vec<u8>, io::Error> {
		Ok(Vec::new())
	}

	fn data_empty(&self) -> bool {
		true
	}
}
