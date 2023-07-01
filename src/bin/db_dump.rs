use std::path::PathBuf;

use palmrs::database::{
	header::DatabaseHeader,
	info::ExtraInfoRecord,
	record::DatabaseRecord,
	DatabaseFormat,
	PalmDatabase,
	PdbWithCategoriesDatabase,
	PrcDatabase,
};
use pretty_hex::{config_hex, HexConfig};
use stable_eyre::eyre::{eyre, Report, WrapErr};
use structopt::StructOpt;

/// Dump the headers, and optionally the record contents, of a Palm OS database file
#[derive(Debug, StructOpt)]
#[structopt(name = "palmrs-db-dump")]
struct Opt {
	/// Print a hex dump of the contents of each database record
	#[structopt(short, long)]
	hexdump_records: bool,

	/// Path to the Palm OS database to dump
	#[structopt(name = "FILE", parse(from_os_str))]
	filename: PathBuf,
}

fn perform_dump_header(header: &DatabaseHeader) -> Result<(), Report> {
	println!(
		"Database name:         {:?}",
		header.name_try_str().unwrap_or("[unknown]")
	);
	println!("Attributes:            {:#X}", { header.attributes });
	println!("Version:               {:#X}", { header.version });
	println!("Creation time:         {}", {
		header.creation_time.strftime("%c (%s)")
	});
	println!("Modification time:     {}", {
		header.modification_time.strftime("%c (%s)")
	});
	println!("Backup time:           {}", {
		header.backup_time.strftime("%c (%s)")
	});
	println!("Modification number:   {:#X}", {
		header.modification_number
	});
	println!("App info offset/ID:    {:#X}", { header.app_info_id });
	println!("Sort info offset/ID:   {:#X}", { header.sort_info_id });
	println!(
		"Type code:             {:?}",
		header.type_code_try_str().unwrap_or("    ")
	);
	println!(
		"Creator code:          {:?}",
		header.creator_code_try_str().unwrap_or("    ")
	);
	println!("Unique ID:             {:#X}", { header.unique_id_seed });
	println!("Next record ID:        {:#X}", { header.next_record_list });
	println!("Record count:          {:#X}", { header.record_count });

	Ok(())
}

fn perform_dump_app_info<T, R>(
	hdr: &DatabaseHeader,
	app_info: &T,
	rec_first: &R,
	data: &[u8],
	opt: &Opt,
) -> Result<(), Report>
where
	T: ExtraInfoRecord,
	R: DatabaseRecord,
{
	log::trace!("app_info = {:#?}", app_info);
	if app_info.data_empty() {
		return Ok(());
	}

	let rec_offset = rec_first.data_offset() as usize;
	let app_info_offset = hdr.app_info_id as usize;
	let app_info_len = rec_offset - app_info_offset;

	let has_categories = if let Some(categories) = app_info.data_item_categories() {
		categories.len() > 0
	} else {
		false
	};

	println!();
	println!(
		"App info: offset={:#X} length={:#X} has_categories={:?}",
		app_info_offset, app_info_len, has_categories,
	);

	if let Some(categories) = app_info.data_item_categories() {
		for cat in categories.iter() {
			println!(
				"  Category {}: name={:?} renamed={:?}",
				cat.category_id,
				cat.name_try_str().unwrap_or(""),
				cat.renamed,
			);
		}
	}

	if opt.hexdump_records {
		println!(
			"{}",
			config_hex(
				&&data[app_info_offset..(app_info_offset + app_info_len)],
				HexConfig {
					title: false,
					..HexConfig::default()
				},
			)
		);
	}

	Ok(())
}

fn perform_dump_record<T>(idx: usize, rec_hdr: &T, rec_data: &[u8], opt: &Opt) -> Result<(), Report>
where
	T: DatabaseRecord,
{
	log::trace!("records[{}] = {:#?}", idx, &rec_hdr);

	let data_offset = rec_hdr.data_offset() as usize;
	let data_len = rec_hdr.data_len().map(|x| x as usize).unwrap_or(0usize);
	let attributes = rec_hdr.attributes().unwrap_or(0);

	println!(
		"Record {}: name={:?} offset={:#X} length={:#X} attributes={:#X}",
		idx,
		rec_hdr.name_str().unwrap_or(""),
		data_offset,
		data_len,
		attributes,
	);

	if opt.hexdump_records {
		println!(
			"{}",
			config_hex(
				&rec_data,
				HexConfig {
					title: false,
					..HexConfig::default()
				},
			)
		);
	}

	Ok(())
}

fn perform_dump<T: DatabaseFormat>(data: &[u8], opt: &Opt) -> Result<(), Report> {
	let database = PalmDatabase::<T>::from_bytes(&data)
		.wrap_err_with(|| format!("Failed to initialize PalmDatabase for {:?}", &opt.filename))?;

	// Dump header
	log::trace!("database.header = {:#?}", &database.header);
	perform_dump_header(&database.header)?;

	// Dump each record, additionally dumping app info before the first record
	for (idx, (rec_hdr, rec_data)) in (0..).zip(database.list_records_resources().iter()) {
		if idx == 0 {
			perform_dump_app_info(&database.header, &database.app_info, rec_hdr, &data, &opt)?;
		}

		println!();
		perform_dump_record(idx, rec_hdr, &rec_data, &opt)?;
	}

	Ok(())
}

fn main() -> Result<(), Report> {
	env_logger::init();
	stable_eyre::install()?;
	let opt = Opt::from_args();

	// Get file extension
	let extension = opt
		.filename
		.extension()
		.ok_or_else(|| eyre!("Couldn't get file extension: {:?}", opt.filename))?
		.to_str()
		.ok_or_else(|| eyre!("Couldn't convert file extension to &str"))?
		.to_lowercase();

	let db_type = match extension.as_str() {
		"prc" => "prc",
		"pdb" | "mobi" => "pdb",
		ext => return Err(eyre!("Unknown database format: {:?}", ext)),
	};

	let content = std::fs::read(&opt.filename)
		.wrap_err_with(|| format!("Failed to read database content from {:?}", &opt.filename))?;

	log::info!(
		"Trying to parse database of type {:?} at path {:?}",
		&db_type,
		&opt.filename
	);

	match db_type {
		"prc" => perform_dump::<PrcDatabase>(&content[..], &opt),
		"pdb" => perform_dump::<PdbWithCategoriesDatabase>(&content[..], &opt),
		_ => unreachable!(),
	}
}
