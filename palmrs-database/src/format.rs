//! Database format helpers

use core::{
	fmt::{self, Debug, Display},
	marker::PhantomData,
};
use std::io;

use crate::{
	header::{DatabaseHeader, DATABASE_HEADER_LENGTH},
	record::{pdb_record::PdbRecordHeader, DatabaseRecord},
};

/// Helper trait for database format types
pub trait DatabaseFormat {
	/// The record header type for this database format
	type RecordHeader: DatabaseRecord;

	/// Returns whether the database is valid as this database format
	fn is_valid(data: &[u8], header: &DatabaseHeader) -> bool;
}

/// Implementation of [`DatabaseFormat`] for PRC databases
pub struct PrcDatabase;
impl DatabaseFormat for PrcDatabase {
	type RecordHeader = PdbRecordHeader;

	fn is_valid(_data: &[u8], header: &DatabaseHeader) -> bool {
		if header.attributes & (1 << 0) == 0 {
			return false;
		}

		true
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

/// A representation of a Palm OS database file
///
/// This uses the [`DatabaseFormat`] trait to allow making access to database records, as well as
/// validity checks on the database content, generic across the PRC and PDB implementations.
#[derive(Clone, PartialEq)]
pub struct PalmDatabase<'a, T: DatabaseFormat> {
	pub header: DatabaseHeader,
	pub records: Vec<(T::RecordHeader, Vec<u8>)>,
	data: &'a [u8],
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

		let mut rec_offset: usize = DATABASE_HEADER_LENGTH;
		let mut records: Vec<(T::RecordHeader, Vec<u8>)> = Vec::new();
		for _idx in 0..header.record_count {
			// parse record header
			let record = T::RecordHeader::from_bytes(&header, &data, rec_offset)?;

			// get record data
			let data_offset = record.data_offset() as usize;
			let data_len = record.data_len().unwrap_or(0) as usize;
			let record_data = Vec::from(&data[data_offset..(data_offset + data_len)]);

			// offset & store
			rec_offset += record.struct_len();
			records.push((record, record_data));
		}

		Ok(Self {
			header,
			records,
			data,
			_marker: PhantomData,
		})
	}
}

impl<'a, T: DatabaseFormat> Debug for PalmDatabase<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PalmDatabase")
			.field("type", &std::any::type_name::<T>())
			.field("header", &self.header)
			.field("records", &self.records)
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
