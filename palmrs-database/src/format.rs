//! Database format helpers

use core::{
	fmt::{self, Debug, Display},
	marker::PhantomData,
};
use std::io;

use crate::{
	header::DatabaseHeader,
	record::{DatabaseRecord, RecordIter},
};

/// Helper trait for database format types
pub trait DatabaseFormat {
	/// The record header type for this database format
	type RecordHeader: DatabaseRecord;

	/// Returns whether the database is valid as this database format
	fn is_valid(data: &[u8], header: &DatabaseHeader) -> bool;
}

/// A representation of a Palm OS database file
///
/// This uses the [`DatabaseFormat`] trait to allow making access to database records, as well as
/// validity checks on the database content, generic across the PRC and PDB implementations.
#[derive(Clone, PartialEq)]
pub struct PalmDatabase<'a, T: DatabaseFormat> {
	pub data: &'a [u8],
	pub header: DatabaseHeader,
	_marker: PhantomData<T>,
}

impl<'a, T: DatabaseFormat> PalmDatabase<'a, T> {
	pub fn from_bytes(data: &'a [u8]) -> Result<Self, io::Error> {
		let header = DatabaseHeader::from_bytes(&data)?;

		if !T::is_valid(&data, &header) {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				"database is not valid",
			));
		}

		Ok(Self {
			data,
			header,
			_marker: PhantomData,
		})
	}

	pub fn iter_records(&self) -> RecordIter<T::RecordHeader> {
		RecordIter::from_database(&self)
	}
}

impl<'a, T: DatabaseFormat> Debug for PalmDatabase<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PalmDatabase")
			.field("type", &std::any::type_name::<T>())
			.field("data", &self.data)
			.field("header", &self.header)
			.finish()
	}
}

impl<'a, T: DatabaseFormat> Display for PalmDatabase<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"PalmDatabase<{}>({:?})",
			std::any::type_name::<T>(),
			self.header.name_try_str().unwrap_or(""),
		)
	}
}
