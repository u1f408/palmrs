//! Individual database record handling
//!
//! This includes the [`DatabaseRecord`] trait, to be implemented by types that represent record
//! headers for a given database type, and the [`pdb_record`] module, which implements that trait
//! for PDB/PRC record and resource header types.

use core::fmt::Debug;
use std::io::{self, Cursor};

use crate::header::DatabaseHeader;

pub mod pdb_record;

/// Helper trait for database record types
pub trait DatabaseRecord: Sized + Debug + sealed::DatabaseRecordHelpers {
	/// Read the record header from the given byte array
	fn from_bytes(hdr: &DatabaseHeader, rdr: &mut Cursor<&[u8]>) -> Result<Self, io::Error>;

	/// Write the record header to a new `Vec<u8>`
	fn to_bytes(&self) -> Result<Vec<u8>, io::Error>;

	/// Return the record's name, if known
	fn name_str(&self) -> Option<&str>;

	/// Return the record's attributes, if known
	fn attributes(&self) -> Option<u32>;

	/// Return the length of the record's data, if known
	fn data_len(&self) -> Option<u32>;
}

mod sealed {

	pub trait DatabaseRecordHelpers {
		/// The length of the record header, in bytes
		fn struct_len(&self) -> usize;

		fn next_entry_data_offset(&self) -> usize;

		fn data_offset(&self) -> u32;
	}
}

pub(crate) use sealed::DatabaseRecordHelpers;
