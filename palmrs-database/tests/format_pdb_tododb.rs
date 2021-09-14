use palmrs_database::{header::DatabaseHeader, record::DatabaseRecord, PalmDatabase, PdbDatabase};
use test_env_log::test;

const EXAMPLE_PDB: &'static [u8] = include_bytes!("../../test-data/ToDoDB.pdb");

#[test]
fn read_header() {
	let header = DatabaseHeader::from_bytes(&EXAMPLE_PDB).unwrap();
	assert_eq!(header.name_try_str().unwrap(), "ToDoDB");
	assert_eq!(header.type_code_try_str().unwrap(), "DATA");
	assert_eq!(header.creator_code_try_str().unwrap(), "todo");
}

#[test]
fn read_database_full() {
	let database = PalmDatabase::<PdbDatabase>::from_bytes(&EXAMPLE_PDB).unwrap();

	// TODO: get "app info" section
	// TODO: get "sort info" section

	// Test record iteration
	for (_idx, (rec_hdr, rec_data)) in (0..).zip(database.records.iter()) {
		assert_eq!(rec_data.len(), rec_hdr.data_len().unwrap_or(0) as usize);
		assert!(rec_hdr.attributes().unwrap_or(0) & 0x40 != 0);
	}
}
