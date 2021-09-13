//! Individual database record handling
//!
//! This includes the [`DatabaseRecord`] trait, to be implemented by types that represent record
//! headers for a given database type; and helpers such as [`RecordIter`].

use core::{fmt::Debug, iter::Iterator, marker::PhantomData};
use std::io;

use crate::{
	header::{DatabaseHeader, DATABASE_HEADER_LENGTH},
	DatabaseFormat,
	PalmDatabase,
};

/// Helper trait for database record types
pub trait DatabaseRecord: Sized + Debug {
	/// The length of the record header, in bytes
	fn struct_len() -> usize;

	/// Read the record header from the given byte array.
	fn from_bytes(data: &[u8], pos: usize) -> Result<Self, io::Error>;

	/// Return the offset, from the start of the database file, of the record's data.
	fn data_offset(&self) -> u32;

	/// Return the length of the record's data, if known.
	fn data_len(&self) -> Option<u32> {
		None
	}
}

/// Iterator over the records in a database
pub struct RecordIter<'a, T: DatabaseRecord> {
	raw_data: &'a [u8],
	header: DatabaseHeader,
	index: usize,
	_marker: PhantomData<T>,
}

impl<'a, T: DatabaseRecord> RecordIter<'a, T> {
	pub fn from_bytes(data: &'a [u8]) -> Result<Self, io::Error> {
		let header = DatabaseHeader::from_bytes(&data[0..DATABASE_HEADER_LENGTH])?;

		let this = Self {
			raw_data: data,
			header,
			index: 0,
			_marker: PhantomData,
		};

		Ok(this)
	}

	pub fn from_database<X: DatabaseFormat>(db: &PalmDatabase<'a, X>) -> Self {
		Self {
			raw_data: db.data.clone(),
			header: db.header.clone(),
			index: 0,
			_marker: PhantomData,
		}
	}
}

impl<'a, T: DatabaseRecord> Iterator for RecordIter<'a, T> {
	type Item = T;

	fn next(&mut self) -> Option<Self::Item> {
		if self.index >= self.header.record_count.into() {
			return None;
		}

		let header_offset = DATABASE_HEADER_LENGTH + (Self::Item::struct_len() * self.index);
		if header_offset > self.raw_data.len() {
			return None;
		}

		let result = match Self::Item::from_bytes(&self.raw_data, header_offset) {
			Ok(res) => res,
			Err(_) => return None,
		};

		self.index += 1;
		Some(result)
	}
}
