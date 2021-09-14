//! Within-conduit environment handling

use core::{cmp::PartialEq, default::Default, fmt::Debug};
use std::{
	collections::HashMap,
	env,
	io::{Error, ErrorKind},
	path::PathBuf,
	str::FromStr,
};

use crate::SyncMode;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct WithinConduitConfig {
	pub sync_mode: SyncMode,
	pub path_local: PathBuf,
	pub path_device: PathBuf,
	pub environment: HashMap<String, String>,
}

impl WithinConduitConfig {
	pub fn from_env(&mut self) -> Result<(), Error> {
		self.sync_mode = match env::var_os("PALMRS_SYNC_MODE") {
			Some(modestr) => match modestr.to_str() {
				Some(modestr) => match SyncMode::from_str(modestr) {
					Ok(mode) => mode,
					Err(_e) => {
						return Err(Error::new(
							ErrorKind::Other,
							"PALMRS_SYNC_MODE not a valid SyncMode",
						));
					}
				},

				None => {
					return Err(Error::new(
						ErrorKind::Other,
						"PALMRS_SYNC_MODE invalid UTF-8?",
					));
				}
			},

			None => {
				return Err(Error::new(ErrorKind::Other, "PALMRS_SYNC_MODE not set"));
			}
		};

		self.path_local = match env::var_os("PALMRS_DATA_LOCAL") {
			Some(pathstr) => PathBuf::from(pathstr),
			None => {
				return Err(Error::new(ErrorKind::Other, "PALMRS_DATA_LOCAL not set"));
			}
		};

		self.path_device = match env::var_os("PALMRS_DATA_DEVICE") {
			Some(pathstr) => PathBuf::from(pathstr),
			None => {
				return Err(Error::new(ErrorKind::Other, "PALMRS_DATA_DEVICE not set"));
			}
		};

		self.environment = {
			let mut conduitenv = HashMap::new();
			let envprefix = match env::var_os("PALMRS_CONFIG_PREFIX") {
				Some(prefixstr) => match prefixstr.to_str() {
					Some(prefix) => String::from(prefix),
					None => {
						return Err(Error::new(
							ErrorKind::Other,
							"PALMRS_CONFIG_PREFIX invalid UTF-8?",
						));
					}
				},

				None => {
					return Err(Error::new(ErrorKind::Other, "PALMRS_CONFIG_PREFIX not set"));
				}
			};

			// Iterate over environment, checking prefix
			for (e_key, e_val) in env::vars().filter(|(k, _)| k.starts_with(&envprefix)) {
				// Strip prefix (unwrapping because we know it has the prefix at this point)
				let key = String::from(e_key.strip_prefix(&envprefix).unwrap());

				// Insert into our environment hashmap
				conduitenv.insert(key, e_val);
			}

			conduitenv
		};

		Ok(())
	}
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithinConduit {
	pub conduit_name: String,
	pub sync_version: String,
	pub config: WithinConduitConfig,
}

impl WithinConduit {
	pub fn new(conduit_name: &str) -> Self {
		Self {
			conduit_name: String::from(conduit_name),
			..Default::default()
		}
	}

	pub fn from_env(mut self) -> Result<Self, Error> {
		self.sync_version = match env::var_os("PALMRS_SYNC_VERSION") {
			Some(verstr) => match verstr.to_str() {
				Some(ver) => String::from(ver),
				None => {
					return Err(Error::new(
						ErrorKind::Other,
						"PALMRS_SYNC_VERSION invalid UTF-8?",
					));
				}
			},

			None => {
				eprintln!("This is the palm.rs sync conduit: {}", &self.conduit_name);
				eprintln!("It looks like you're trying to run this executable directly.");
				eprintln!("Sync conduits are run as part of 'palmrs-sync', and can't be directly executed.");
				eprintln!("Stubbornly refusing to continue, sorry!");

				return Err(Error::new(ErrorKind::Other, "PALMRS_SYNC_VERSION not set"));
			}
		};

		self.config.from_env()?;

		Ok(self)
	}
}

impl Default for WithinConduit {
	fn default() -> Self {
		Self {
			conduit_name: String::new(),
			sync_version: String::from("0.0.0"),
			config: Default::default(),
		}
	}
}
