//! Support for Palm HotSync, with pluggable sync conduits

use core::{
	cmp::PartialEq,
	default::Default,
	fmt::{self, Debug, Display},
	str::FromStr,
};

pub mod conduit;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SyncMode {
	KeepLocal,
	KeepDevice,
	Merge,
}

impl Default for SyncMode {
	fn default() -> Self {
		Self::Merge
	}
}

impl FromStr for SyncMode {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s.to_ascii_lowercase().as_str() {
			"keep-local" | "keeplocal" | "local" => Ok(Self::KeepLocal),
			"keep-device" | "keepdevice" | "device" => Ok(Self::KeepDevice),
			"merge" => Ok(Self::Merge),

			_ => Err(String::from(s)),
		}
	}
}

impl Display for SyncMode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::KeepLocal => write!(f, "keep-local"),
			Self::KeepDevice => write!(f, "keep-device"),
			Self::Merge => write!(f, "merge"),
		}
	}
}
