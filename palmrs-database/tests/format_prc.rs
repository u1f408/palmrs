use palmrs_database::{
	format_prc::{PrcDatabase, PrcRecordHeader},
	header::DatabaseHeader,
	record::{DatabaseRecord, RecordIter},
	PalmDatabase,
};
use test_env_log::test;

const EXAMPLE_PRC: &'static [u8] = include_bytes!("data/hello.prc");

#[test]
fn read_header() {
	let header = DatabaseHeader::from_bytes(&EXAMPLE_PRC).unwrap();
	assert_eq!(header.name_try_str().unwrap(), "Hello, World");
}

#[test]
fn iterate_records() {
	let iterator = RecordIter::<PrcRecordHeader>::from_bytes(&EXAMPLE_PRC).unwrap();
	for record in iterator {
		assert!(record.data_offset() > 0);
		assert!(record.data_len().is_some());
	}
}

#[test]
fn read_database_full() {
	let database = PalmDatabase::<PrcDatabase>::from_bytes(&EXAMPLE_PRC).unwrap();
	eprintln!("{}", &database);

	// Test record iteration
	for record in database.iter_records() {
		// Get a slice of the record content
		let (data_offset, data_len) = (
			record.data_offset() as usize,
			record.data_len().unwrap() as usize,
		);
		let record_data = &database.data[data_offset..(data_offset + data_len)];
		assert!(record_data.len() > 0);

		eprintln!("{} - {:?}", &record, record_data);
	}
}
