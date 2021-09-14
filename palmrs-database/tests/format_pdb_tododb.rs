use palmrs_database::{
	header::DatabaseHeader,
	info::ExtraInfoRecord,
	record::DatabaseRecord,
	PalmDatabase,
	PdbWithCategoriesDatabase,
};
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
	let database = PalmDatabase::<PdbWithCategoriesDatabase>::from_bytes(&EXAMPLE_PDB).unwrap();

	// Check for categories in the app info record
	if let Some(categories) = database.app_info.data_item_categories() {
		for cat in categories.iter() {
			assert!(cat.category_id < 16);

			// make sure category 0 is "unfiled"
			if cat.category_id == 0 {
				assert_eq!(cat.name_try_str().ok(), Some("Unfiled"));
			}
		}
	}

	// Test record iteration
	for (_idx, (rec_hdr, rec_data)) in (0..).zip(database.records.iter()) {
		assert_eq!(rec_data.len(), rec_hdr.data_len().unwrap_or(0) as usize);
		assert!(rec_hdr.attributes().unwrap_or(0) & 0x40 != 0);
	}
}
