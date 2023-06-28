//! Database format helpers

use core::{
	fmt::{self, Debug, Display},
	marker::PhantomData,
};
use std::io::{self, Cursor, Write};

use crate::{
	header::{DatabaseHeader, DATABASE_HEADER_LENGTH},
	info::{category::AppInfoCategories, ExtraInfoRecord, NullExtraInfo},
	record::{pdb_record::PdbRecordHeader, DatabaseRecord},
};

// add 2 bytes of padding for < os3.5 compatible PRCs
const COMPAT_PADDING: usize = 2;

/// Helper trait for database format types
pub trait DatabaseFormat {
	/// The record header type for this database format
	type RecordHeader: DatabaseRecord;

	/// The type of the app info record
	type AppInfoRecord: ExtraInfoRecord;

	/// Returns whether the database is valid as this database format
	fn is_valid(data: &[u8], header: &DatabaseHeader) -> bool;
}

/// Implementation of [`DatabaseFormat`] for PRC databases
pub struct PrcDatabase;
impl DatabaseFormat for PrcDatabase {
	type RecordHeader = PdbRecordHeader;
	type AppInfoRecord = NullExtraInfo;

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
	type AppInfoRecord = NullExtraInfo;

	fn is_valid(_data: &[u8], header: &DatabaseHeader) -> bool {
		if header.attributes & (1 << 0) != 0 {
			return false;
		}

		true
	}
}

/// Implementation of [`DatabaseFormat`] for PDB databases that contain category information
pub struct PdbWithCategoriesDatabase;
impl DatabaseFormat for PdbWithCategoriesDatabase {
	type RecordHeader = PdbRecordHeader;
	type AppInfoRecord = AppInfoCategories;

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
	pub app_info: T::AppInfoRecord,
	application_reserved: Vec<u8>,
	pub records: Vec<(T::RecordHeader, Vec<u8>)>,
	pub(crate) original_data: &'a [u8],
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

		let app_info = T::AppInfoRecord::from_bytes(&header, &data, header.app_info_id as usize)?;

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

		let application_reserved: Vec<u8>;
		if (header.record_count > 0)
			&& ((records[0].0.data_offset() as usize)
				> rec_offset + T::AppInfoRecord::SIZE + COMPAT_PADDING)
		{
			let first_record_data_start = records[0].0.data_offset() as usize;
			application_reserved = Vec::from(
				&data[(rec_offset + T::AppInfoRecord::SIZE + COMPAT_PADDING)
					..(first_record_data_start)],
			)
		} else {
			application_reserved = Vec::new()
		};

		Ok(Self {
			header,
			app_info,
			application_reserved,
			records,
			original_data: data,
			_marker: PhantomData,
		})
	}

	pub fn to_bytes(self) -> std::io::Result<Vec<u8>> {
		let mut cursor = Cursor::new(self.header.to_bytes()?);
		cursor.set_position(cursor.get_ref().len() as u64);

		for (record_header, _) in self.records.iter() {
			cursor.write(&record_header.to_bytes()?)?;
		}

		cursor.write(&[0_u8; COMPAT_PADDING])?;

		cursor.write(&self.app_info.to_bytes()?)?;

		// add any reserved data
		cursor.write(&self.application_reserved)?;

		for (_, record_data) in self.records.iter() {
			cursor.write(&record_data)?;
		}

		Ok(cursor.into_inner())
	}
}

impl<'a, T: DatabaseFormat> Debug for PalmDatabase<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("PalmDatabase")
			.field("type", &std::any::type_name::<T>())
			.field("header", &self.header)
			.field("app_info", &self.app_info)
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
