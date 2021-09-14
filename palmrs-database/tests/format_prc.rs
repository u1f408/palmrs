use palmrs_database::{header::DatabaseHeader, record::DatabaseRecord, PalmDatabase, PrcDatabase};
use test_env_log::test;

const EXAMPLE_PRC: &'static [u8] = include_bytes!("../../test-data/hello-v1.prc");

#[test]
fn read_header() {
	let header = DatabaseHeader::from_bytes(&EXAMPLE_PRC).unwrap();
	assert_eq!(header.name_try_str().unwrap(), "Hello, World");
}

#[test]
fn read_database_full() {
	let database = PalmDatabase::<PrcDatabase>::from_bytes(&EXAMPLE_PRC).unwrap();

	// Test record iteration
	for (_idx, (rec_hdr, rec_data)) in (0..).zip(database.records.iter()) {
		assert_eq!(rec_data.len(), rec_hdr.data_len().unwrap_or(0) as usize);
	}
}
