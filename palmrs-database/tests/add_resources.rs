use palmrs_database::{
	record::{
		pdb_record::{PdbRecordHeader, RecordAttributes},
		DatabaseRecord,
	},
	DatabaseFormat,
	PalmDatabase,
	PdbDatabase,
	PdbWithCategoriesDatabase,
	PrcDatabase,
};

const EXAMPLE_PRC: &'static [u8] = include_bytes!("../../test-data/hello-v1.prc");
const EXAMPLE_PDB: &'static [u8] = include_bytes!("../../test-data/ToDoDB.pdb");
const MANUAL_PDB: &'static [u8] = include_bytes!("../../test-data/tWmanual.pdb");

fn test_db<T: DatabaseFormat>(src_bytes: &'static [u8])
where
	<T as DatabaseFormat>::RecordHeader: PartialEq<PdbRecordHeader>,
{
	let mut database = PalmDatabase::<T>::from_bytes(src_bytes).unwrap();

	let test_str = "sphinx of black quartz, judge my vow";

	// test adding a record
	let record_id = database.insert_record(RecordAttributes::default(), test_str.as_bytes());
	let (_, recovered_data) = database
		.list_records_resources()
		.into_iter()
		.find(|(hdr, _)| hdr.unique_id() == Some(record_id))
		.unwrap();
	assert_eq!(
		String::from_utf8(recovered_data.to_vec()).unwrap(),
		test_str
	);

	// test adding a resource
	let mut test_name: [u8; 4] = [0_u8; 4];
	test_name[0..4].copy_from_slice("test".as_bytes());
	let resource_id = database.insert_resource(&test_name, test_str.as_bytes());
	let (hdr, recovered_data) = database
		.list_records_resources()
		.into_iter()
		.find(|(hdr, _)| hdr.resource_id() == Some(resource_id))
		.unwrap();
	assert_eq!(
		String::from_utf8(recovered_data.to_vec()).unwrap(),
		test_str
	);

	assert_eq!(hdr.name_str().map(str::as_bytes).unwrap(), test_name);
}

#[test]
fn test_records_all_db_types() {
	test_db::<PrcDatabase>(&EXAMPLE_PRC);
	test_db::<PdbDatabase>(&MANUAL_PDB);
	test_db::<PdbWithCategoriesDatabase>(&EXAMPLE_PDB);
}
