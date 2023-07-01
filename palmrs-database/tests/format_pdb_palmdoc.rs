use std::io::Cursor;

use palmrs_database::{header::DatabaseHeader, record::DatabaseRecord, PalmDatabase, PdbDatabase};
use test_env_log::test;

const EXAMPLE_PDB: &'static [u8] = include_bytes!("../../test-data/tWmanual.pdb");

#[test]
fn read_header() {
	let header = DatabaseHeader::from_bytes(&mut Cursor::new(&EXAMPLE_PDB)).unwrap();
	assert_eq!(header.name_try_str().unwrap(), "tWmanual");
	assert_eq!(header.type_code_try_str().unwrap(), "TEXt");
	assert_eq!(header.creator_code_try_str().unwrap(), "REAd");
}

#[test]
fn read_database_full() {
	let database = PalmDatabase::<PdbDatabase>::from_bytes(&EXAMPLE_PDB).unwrap();

	// TODO: get "app info" section
	// TODO: get "sort info" section

	// Test record iteration
	for (_idx, (rec_hdr, rec_data)) in database.list_records_resources().iter().enumerate() {
		assert_eq!(rec_data.len(), rec_hdr.data_len().unwrap_or(0) as usize);
		assert!(rec_hdr.attributes().unwrap_or(0) & 0x40 != 0);
	}

	let bytes = database.to_bytes().unwrap();

	assert_eq!(bytes.len(), EXAMPLE_PDB.len());
	assert_eq!(&EXAMPLE_PDB, &bytes);
}
