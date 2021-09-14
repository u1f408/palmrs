//! Sync conduit handling
//!
//! ## Conduit specification
//!
//! Conduits are implemented as external executables, which are called at sync time to do one of
//! three things:
//!
//! - Replace the device's copy of the data with the local copy of the data
//! - Replace the local copy of the data with the data from the device
//! - Merge the local copy of the data with the device's copy of the data
//!
//! The sync mode, along with other conduit data, is passed in the form of environment variables to
//! the conduit process.
//!
//! ### Environment variables
//!
//! Conduits are always called with the following variables set in their environment:
//!
//! - `PALMRS_SYNC_VERSION` - palmrs-sync version number
//! - `PALMRS_SYNC_MODE` - sync mode (one of `keep-local`, `keep-device`, or `merge`)
//! - `PALMRS_DATA_LOCAL` - path to the local data directory for this conduit
//! - `PALMRS_DATA_DEVICE` - path to a directory containing the entire set of Palm OS database
//!   files pulled from the device at the beginning of the sync process
//!
//! In addition, conduits are called with a parsed version of their conduit-specific configuration,
//! pulled from the palm.rs configuration for the specific device that is being synced, in their
//! environment. The names of all of these environment variables are prefixed, and that prefix is
//! passed to the conduit in the `PALMRS_CONFIG_PREFIX` environment variable.
//!
//! For example, a test conduit could be called with the following environment:
//!
//! ```shell
//! PALMRS_SYNC_VERSION="0.1.0-dev.1"
//! PALMRS_SYNC_MODE="keep-device"
//! PALMRS_DATA_LOCAL="/home/user/Documents/palm.rs/conduits/conduit-test"
//! PALMRS_DATA_DEVICE="/tmp/palmrs-device-data.27b862fa.tmp"
//! PALMRS_CONFIG_PREFIX="CONDUIT_TEST__"
//! CONDUIT_TEST__HELLO_WORLD="Hello, world!"
//! ```
//!
//! ### Local data
//!
//! Conduits are free to store whatever data they would like, in whatever format they would like,
//! in their local data directory (the path to which is specified in the `PALMRS_DATA_LOCAL`
//! environment variable passed to the conduit).
//!
//! It is _recommended_, but not _required_, that conduits store their data in a format that is
//! easily editable by a human - either directly with a text editor, or by using already available
//! tools.
//!
//! ### Device data
//!
//! All of the Palm OS database files that have been synced from the device, regardless of whether
//! or not the specific conduit needs them, will be available in the directory specified by the
//! `PALMRS_DATA_DEVICE` environment variable.
//!
//! The data available in that directory may not be the result of a "full" (i.e. "backup") sync
//! with the device - in that case, the partial data from the current sync will be overlaid on top
//! of the data from the last "full" sync.
//!
//! When the conduit is operating in `keep-local` or `merge` modes, any data written back to any of
//! the databases in the `PALMRS_DATA_DEVICE` directory (or, any new database files that are added
//! to that directory) will be synced back to the device. When the conduit is operating in
//! `keep-device` mode, the conduit _MAY NOT_ make changes to the databases in that directory.

use core::{cmp::PartialEq, fmt::Debug};
use std::{collections::HashMap, env, ffi::OsString, path::PathBuf};

use subprocess::{self, Popen, PopenConfig, Redirection};

use crate::SyncMode;

mod within;
pub use self::within::{WithinConduit, WithinConduitConfig};

/// Conduit call handler
#[derive(Debug, Clone, PartialEq)]
pub struct ConduitHandler {
	pub sync_mode: SyncMode,
	pub conduit_name: String,
	pub conduit_config: HashMap<OsString, OsString>,
	pub path_local: PathBuf,
	pub path_device: PathBuf,
}

impl ConduitHandler {
	pub fn new(conduit_name: &str, sync_mode: SyncMode) -> Self {
		Self {
			sync_mode,
			conduit_name: String::from(conduit_name),
			conduit_config: HashMap::new(),
			path_local: PathBuf::new(),
			path_device: PathBuf::new(),
		}
	}

	pub fn make_config_prefix(&self) -> OsString {
		let mut prefix = self
			.conduit_name
			.replace("palmrs-", "")
			.replace("-", "_")
			.replace("__", "_")
			.to_uppercase();

		prefix.push_str("__");
		prefix.into()
	}

	pub fn make_argv(&self) -> Vec<OsString> {
		vec![self.conduit_name.clone().into()]
	}

	pub fn make_environment(&self) -> Vec<(OsString, OsString)> {
		let mut env: Vec<(OsString, OsString)> = Vec::new();
		let mangled_prefix = self.make_config_prefix();

		// Inherit a few things from the current environment
		env.push((
			"HOME".into(),
			env::var_os("HOME").unwrap_or_else(OsString::new),
		));
		env.push((
			"PATH".into(),
			env::var_os("PATH").unwrap_or_else(OsString::new),
		));

		// Inherit Rust logging & backtrace settings, if they're set
		if let Some(var_rust_log) = env::var_os("RUST_LOG") {
			env.push(("RUST_LOG".into(), var_rust_log.into()));
		}
		if let Some(var_rust_backtrace) = env::var_os("RUST_BACKTRACE") {
			env.push(("RUST_BACKTRACE".into(), var_rust_backtrace.into()));
		}

		// Generic configuration
		env.push((
			"PALMRS_SYNC_VERSION".into(),
			env!("CARGO_PKG_VERSION").into(),
		));
		env.push((
			"PALMRS_SYNC_MODE".into(),
			format!("{}", self.sync_mode).into(),
		));
		env.push((
			"PALMRS_DATA_LOCAL".into(),
			self.path_local.as_os_str().into(),
		));
		env.push((
			"PALMRS_DATA_DEVICE".into(),
			self.path_device.as_os_str().into(),
		));

		// Conduit-specific configuration keys
		env.push(("PALMRS_CONFIG_PREFIX".into(), mangled_prefix.clone()));
		for (cfgkey, cfgval) in &self.conduit_config {
			let mut key = mangled_prefix.clone();
			key.push(cfgkey.to_ascii_uppercase());

			env.push((key, cfgval.clone()));
		}

		env
	}

	pub fn popen(&self, stdout: Redirection, stderr: Redirection) -> subprocess::Result<Popen> {
		let config = PopenConfig {
			stdout,
			stderr,
			env: Some(self.make_environment()),
			..Default::default()
		};

		Popen::create(&self.make_argv(), config)
	}
}
