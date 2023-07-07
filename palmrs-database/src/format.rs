//! Database format helpers

use core::{
	fmt::{self, Debug, Display},
	marker::PhantomData,
};
use std::{
	collections::HashSet,
	io::{self, Cursor, Read, Write},
};

use crate::{
	header::DatabaseHeader,
	info::{category::AppInfoCategories, ExtraInfoRecord, NullExtraInfo},
	record::{
		pdb_record::{PdbRecordHeader, RecordAttributes},
		DatabaseRecord,
		DatabaseRecordHelpers,
	},
};

// add 2 bytes of padding for < os3.5 compatible PRCs
const COMPAT_PADDING_LEN: usize = 2;

/// Helper trait for database format types
pub trait DatabaseFormat {
	const USES_COMPAT_PADDING: bool;

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
	const USES_COMPAT_PADDING: bool = false;
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
	const USES_COMPAT_PADDING: bool = false;
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
	const USES_COMPAT_PADDING: bool = true;
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

	/// Also called sortInfo
	application_reserved: Vec<u8>,

	/// record headers together with their contained data. This is for convenience,
	/// and does not match the on-disk layout
	records: Vec<(T::RecordHeader, Vec<u8>)>,
	pub(crate) original_data: &'a [u8],
	_marker: PhantomData<T>,
}

impl<'a, T: DatabaseFormat> PalmDatabase<'a, T> {
	pub fn from_bytes(data: &'a [u8]) -> Result<Self, io::Error> {
		let mut rdr = Cursor::new(data);
		let header = DatabaseHeader::from_bytes(&mut rdr)?;

		if !T::is_valid(&data, &header) {
			return Err(io::Error::new(
				io::ErrorKind::Other,
				"database is not valid",
			));
		}

		let mut record_headers: Vec<T::RecordHeader> = Vec::new();
		for _idx in 0..header.record_count {
			// parse record header
			let record = T::RecordHeader::from_bytes(&header, &mut rdr)?;

			// store
			record_headers.push(record);
		}

		if T::USES_COMPAT_PADDING {
			rdr.read_exact(&mut [0_u8; COMPAT_PADDING_LEN])?;
		}

		let app_info = T::AppInfoRecord::from_bytes(&header, &mut rdr)?;

		let application_reserved: Vec<u8>;
		let app_info_end = rdr.position() as usize;
		let first_record_data_start = record_headers[0].data_offset() as usize;
		if (header.record_count > 0) && (first_record_data_start > app_info_end) {
			let mut buf = vec![0_u8; first_record_data_start - app_info_end];
			rdr.read_exact(&mut buf)?;
			application_reserved = buf;
		} else {
			application_reserved = Vec::new()
		};

		let mut records: Vec<(T::RecordHeader, Vec<u8>)> = Vec::new();
		for record_header in record_headers {
			let record_data_start = record_header.data_offset();
			let record_data_end = record_data_start + record_header.data_len().unwrap_or(0);
			let mut record_data = vec![0_u8; (record_data_end - record_data_start) as usize];
			rdr.read_exact(&mut record_data)?;
			records.push((record_header, record_data));
		}

		Ok(Self {
			header,
			app_info,
			application_reserved,
			records,
			original_data: data,
			_marker: PhantomData,
		})
	}

	pub fn to_bytes(&self) -> std::io::Result<Vec<u8>> {
		let mut cursor = Cursor::new(self.header.to_bytes()?);
		cursor.set_position(cursor.get_ref().len() as u64);

		for (record_header, _) in self.records.iter() {
			cursor.write(&record_header.to_bytes()?)?;
		}

		if T::USES_COMPAT_PADDING {
			cursor.write(&[0_u8; COMPAT_PADDING_LEN])?;
		}

		cursor.write(&self.app_info.to_bytes()?)?;

		// add any reserved data
		cursor.write(&self.application_reserved)?;

		for (_, record_data) in self.records.iter() {
			cursor.write(&record_data)?;
		}

		Ok(cursor.into_inner())
	}

	pub fn list_records_resources(&self) -> &[(T::RecordHeader, Vec<u8>)] {
		&self.records
	}

	/// Create a new record in the database, returning the ID of the new record
	pub fn insert_record(&mut self, attributes: RecordAttributes, data: &[u8]) -> u32 {
		let headers = self
			.list_records_resources()
			.into_iter()
			.map(|(hdr, _)| hdr)
			.collect::<Vec<_>>();
		let used_ids = headers
			.clone()
			.into_iter()
			.filter_map(DatabaseRecord::unique_id)
			.collect::<HashSet<_>>();
		let unique_id = (0..).find(|x| !used_ids.contains(x)).unwrap();

		let record = {
			let record_header_offset = headers
				.iter()
				.last()
				.map(|hdr| hdr.data_offset() + hdr.data_len().unwrap_or(0))
				.unwrap_or_else(|| self.to_bytes().unwrap().len() as u32);
			let data_len = match data.len() {
				0 => None,
				other => Some(other as u32),
			};

			T::RecordHeader::construct_record(attributes, unique_id, record_header_offset, data_len)
		};

		self.records.push((record, data.to_owned()));

		unique_id
	}

	/// Create a new resource in the database, returning the ID of the new record
	pub fn insert_resource(&mut self, name: &[u8; 4], data: &[u8]) -> u16 {
		let headers = self
			.list_records_resources()
			.into_iter()
			.map(|(hdr, _)| hdr)
			.collect::<Vec<_>>();
		let used_ids = headers
			.clone()
			.into_iter()
			.filter_map(DatabaseRecord::resource_id)
			.collect::<HashSet<_>>();
		let unique_id = (0..).find(|x| !used_ids.contains(x)).unwrap();

		let record = {
			let record_header_offset = headers
				.iter()
				.last()
				.map(|hdr| hdr.data_offset() + hdr.data_len().unwrap_or(0))
				.unwrap_or_else(|| self.to_bytes().unwrap().len() as u32);
			let data_len = match data.len() {
				0 => None,
				other => Some(other as u32),
			};

			T::RecordHeader::construct_resource(name, unique_id, record_header_offset, data_len)
		};

		self.records.push((record, data.to_owned()));

		unique_id
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
