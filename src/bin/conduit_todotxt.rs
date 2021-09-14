//! palmrs-conduit-todotxt: Palm OS "Tasks" app <-> `todo.txt` sync conduit

use std::{
	fs,
	io::{BufRead, Cursor},
	path::Path,
	str,
};

use byteorder::{BigEndian, ReadBytesExt};
use palmrs::{
	database::{
		info::{category::CATEGORY_ATTRIBUTE_MASK, ExtraInfoRecord},
		record::DatabaseRecord,
		PalmDatabase,
		PdbWithCategoriesDatabase,
	},
	sync::{
		conduit::{ConduitRequirements, WithinConduit},
		SyncMode,
	},
};
use stable_eyre::eyre::{eyre, Report, WrapErr};

/// A single to-do task
#[derive(Debug, Clone, PartialEq)]
pub struct ToDoTask {
	/// Task body
	pub task_text: String,

	/// Extended task note text
	pub note_text: Option<String>,

	/// Category name
	///
	/// Uses the "unfiled" category if None.
	pub category: Option<String>,

	/// Task priority (between 1 and 5)
	pub priority: u8,

	/// Has the task been completed?
	pub completed: bool,

	/// Task due date, if defined, as a `(year, month, day)` tuple
	pub due_date: Option<(u16, u8, u8)>,
}

impl ToDoTask {
	/// Return this task as a `todo.txt` formatted String
	pub fn as_todotxt_entry(&self) -> String {
		let mut s = String::new();

		// completed?
		if self.completed {
			s.push_str("x ");
		}

		// priority
		s.push('(');
		s.push((self.priority + 64) as char);
		s.push(')');
		s.push(' ');

		// due date?
		if let Some((year, month, day)) = &self.due_date {
			s.push_str(&format!("{:04}-{:02}-{:02} ", year, month, day));
		}

		// task text
		s.push_str(&self.task_text.replace("\n", " "));

		// category?
		if let Some(category) = &self.category {
			s.push_str(&format!(" @{}", category.replace(" ", "_")));
		}

		// note text?
		if let Some(note_text) = &self.note_text {
			s.push_str(&format!(" note={:?}", note_text));
		}

		s
	}
}

/// Parse tasks out of a `ToDoDB` database
pub fn device_database_parse(db_path: &Path) -> Result<Vec<ToDoTask>, Report> {
	let db_content =
		fs::read(db_path).wrap_err_with(|| eyre!("Failed to read database: {:?}", db_path))?;
	let database = PalmDatabase::<PdbWithCategoriesDatabase>::from_bytes(&db_content)
		.wrap_err_with(|| eyre!("Failed to parse device ToDoDB: {:?}", db_path))?;

	let categories: Vec<(u8, String)> = {
		let mut categories = Vec::new();
		if let Some(db_cats) = database.app_info.data_item_categories() {
			for cat in db_cats.iter() {
				categories.push((
					cat.category_id,
					String::from(cat.name_try_str().unwrap_or("ErrCategory")),
				));
			}
		}

		categories
	};

	let mut tasks = Vec::new();
	'dbtaskparse: for (idx, (rec_hdr, rec_data)) in (0..).zip(database.records.iter()) {
		log::trace!("records[{}].rec_hdr = {:?}", idx, &rec_hdr);
		if rec_hdr.data_len().unwrap_or(0) == 0 {
			break 'dbtaskparse;
		}

		// Get category from record attributes
		let rec_attributes = rec_hdr.attributes().unwrap_or(0u32);
		let category = {
			let mut catname: Option<String> = None;
			let catid = ((rec_attributes & 0xFF) as u8) & CATEGORY_ATTRIBUTE_MASK;
			'catsearch: for (x_catid, x_catname) in categories.iter() {
				if catid == *x_catid {
					catname = Some(x_catname.clone());
					break 'catsearch;
				}
			}

			catname
		};

		// Parse out the due date, priority, and completion flag
		let mut cursor = Cursor::new(&rec_data);
		let due_date = {
			let mut due_date: Option<(u16, u8, u8)> = None;
			let raw_date = cursor.read_u16::<BigEndian>()?;
			if raw_date != 0xFFFF {
				due_date = Some((
					((raw_date >> 9) & 0x007F) + 1904,
					((raw_date >> 5) & 0x000F) as u8,
					(raw_date & 0x001F) as u8,
				));
			}

			due_date
		};

		let priority = cursor.read_u8()?;
		let completed = priority & 0x80 != 0;
		let priority = priority & 0x7F;

		// Read out the task text
		let task_text = {
			let mut buf: Vec<u8> = Vec::new();
			let count = cursor
				.read_until(0x00, &mut buf)
				.expect("cursor read fail?");

			str::from_utf8(&buf[0..(count - 1)])
				.wrap_err("UTF-8 conversion failed")?
				.to_string()
		};

		// Read out the note text
		let note_text = {
			let mut buf: Vec<u8> = Vec::new();
			let count = cursor
				.read_until(0x00, &mut buf)
				.expect("cursor read fail?");

			if count > 1 {
				let s = str::from_utf8(&buf[0..(count - 1)])
					.wrap_err("UTF-8 conversion failed")?
					.to_string();

				Some(s)
			} else {
				None
			}
		};

		// And construct our task object!
		tasks.push(ToDoTask {
			task_text,
			note_text,
			category,
			priority,
			completed,
			due_date,
		});
	}

	Ok(tasks)
}

/// Perform a one-way conversion from the Palm OS `ToDoDB` to the `todo.txt` format
///
/// If `conduit.config.environment["SINGLE_FILE"]` is set to `"1"`, this method will store both
/// incomplete and complete tasks in the same file (named `todo.txt`). If it is set to any other
/// value (the default is `"0"`), this method will store incomplete tasks in `todo.txt`, and
/// complete tasks in `done.txt`.
pub fn palm_to_todotxt(conduit: &WithinConduit) -> Result<(), Report> {
	log::info!("palm_to_todotxt: parsing Palm OS tasks database");

	let mut device_database = conduit.config.path_device.clone();
	device_database.push("ToDoDB.pdb");
	let device_tasks = device_database_parse(&device_database)?;
	log::trace!("device_tasks = {:#?}", &device_tasks);

	// Collate todo.txt entries by completion status
	let mut todotxt_incomplete: Vec<String> = Vec::new();
	let mut todotxt_completed: Vec<String> = Vec::new();
	for task in device_tasks.iter() {
		if task.completed {
			todotxt_completed.push(task.as_todotxt_entry());
		} else {
			todotxt_incomplete.push(task.as_todotxt_entry());
		}
	}

	log::info!(
		"palm_to_todotxt: {} incomplete, {} complete tasks",
		todotxt_incomplete.len(),
		todotxt_completed.len(),
	);

	// Get paths to `todo.txt` and `done.txt`
	let mut todotxt_path = conduit.config.path_local.clone();
	todotxt_path.push("todo.txt");
	log::debug!("todotxt_path = {:?}", &todotxt_path);
	let mut donetxt_path = conduit.config.path_local.clone();
	donetxt_path.push("done.txt");
	log::debug!("donetxt_path = {:?}", &donetxt_path);

	// Are we in single-file mode?
	let single_file = conduit
		.config
		.environment
		.get("SINGLE_FILE")
		.unwrap_or(&String::from("0"))
		.as_str()
		== "1";

	if single_file {
		// If we're in single file mode, write incomplete _then_ completed to `todo.txt`
		log::info!("palm_to_todotxt: operating in single-file mode");

		fs::write(&todotxt_path, {
			let mut v = Vec::new();
			for task in todotxt_incomplete.iter() {
				v.push(task.clone());
			}
			for task in todotxt_completed.iter() {
				v.push(task.clone());
			}

			v.join("\n")
		})?;
	} else {
		// If we're not in single-file mode, write incomplete to `todo.txt`, and completed to
		// `done.txt`
		log::info!("palm_to_todotxt: operating in dual-file mode");

		fs::write(&todotxt_path, todotxt_incomplete.join("\n"))?;
		fs::write(&donetxt_path, todotxt_completed.join("\n"))?;
	}

	log::info!("palm_to_todotxt: wrote our todo.txt file(s) :)");
	Ok(())
}

/// Main entrypoint
pub fn main() -> Result<(), Report> {
	env_logger::init();
	stable_eyre::install()?;

	let conduit = WithinConduit::new(
		"palmrs-conduit-todotxt",
		ConduitRequirements::new()
			.with_databases(&["ToDoDB"])
			.finish(),
	)
	.from_env()
	.wrap_err("WithinConduit build failed")?;
	log::trace!("conduit = {:#?}", &conduit);

	match conduit.config.sync_mode {
		SyncMode::KeepDevice => palm_to_todotxt(&conduit),
		_ => unimplemented!(),
	}
}
