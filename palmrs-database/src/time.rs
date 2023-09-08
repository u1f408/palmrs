//! `PalmTimestamp` type & conversion methods
//!
//! Palm OS has two epochs used for storing time - the "old Palm epoch" (seconds since 1904-01-01
//! 00:00:00), and the standard UNIX epoch (seconds since 1970-01-01 00:00:00). If the "old Palm
//! epoch" is used for storing the time, it is stored as an _unsigned_ 32-bit integer, but if the
//! UNIX epoch is used, it is stored as a _signed_ 32-bit integer.
//!
//! This module provides the [`PalmTimestamp`] helper type, which can be used within larger data
//! structures to provide automatic timestamp format conversion. This module also provides various
//! helper methods for timestamp format detection, and conversion between the timestamp formats.

use core::{
	convert::TryFrom,
	fmt::{self, Debug, Display},
};

use chrono::{TimeZone, Utc};

/// The number of seconds between the two Palm OS timestamp epochs.
///
/// Adding this constant to an "old Palm epoch" timestamp will give you a UNIX epoch timestamp.
///
/// The value of this constant differs from the value used in some other Palm OS utilities
/// (specifically, [palmdump](https://www.fourmilab.ch/palm/palmdump)). I am not entirely sure why
/// palmdump uses the value it does. For clarification's sake, here is how the value provided here
/// was calculated (as a Python 3 snippet):
///
/// ```python
/// from datetime import datetime
/// print("%d" % abs(datetime.timestamp(datetime(1904, 1, 1))))
/// ```
pub const SECONDS_BETWEEN_PALM_EPOCHS: u32 = 2082886200;

/// Type representing a Palm OS timestamp
///
/// The raw data contained within this struct can be _either_ of the Palm OS timestamp formats
/// (seconds since the UNIX epoch, or seconds since the "old Palm epoch," see the module
/// documentation for more info), so that this type can be embedded directly within raw data
/// structures.
///
/// To get a usable timestamp from this type, the [`as_unix_ts`][PalmTimestamp::as_unix_ts] method
/// can be called to return the number of seconds since the Unix epoch.
#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(C, packed)]
pub struct PalmTimestamp(pub u32);

impl PalmTimestamp {
	/// Return the timestamp as the seconds since the UNIX epoch
	///
	/// If the containing type contains an "old Palm epoch" timestamp, as may be the case if the
	/// timestamp was loaded from a PRC/PDB file, this method will perform an implicit conversion
	/// to the UNIX epoch.
	pub fn as_unix_ts(&self) -> i32 {
		if is_palm_epoch(self.0) {
			return palm_ts_to_unix_ts(self.0);
		}

		self.0 as i32
	}

	/// Return the timestamp as a `strftime`-formatted string
	///
	/// This uses the formatting specifiers from [`chrono::format::strftime`].
	pub fn strftime(&self, format: &str) -> String {
		let datetime = Utc.timestamp_opt(self.as_unix_ts() as i64, 0).unwrap();
		datetime.format(format).to_string()
	}
}

impl Default for PalmTimestamp {
	fn default() -> Self {
		// 1970-01-01 00:00:00 as a UNIX epoch timestamp
		Self(0u32)
	}
}

impl Display for PalmTimestamp {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "PalmTimestamp({})", self.as_unix_ts())
	}
}

/// Check if the given timestamp is using the "old Palm epoch"
///
/// This uses the incredibly simple heuristic of "is the top bit set?" -- this works because the
/// top bit will always be clear if we're dealing with a signed integer (which is the case if the
/// timestamp is using the UNIX epoch), and if the timestamp is using the old Palm epoch, the top
/// bit will always be set if the timestamp is a date occurring after some time in 1972.
///
/// Palm OS definitely wasn't around in 1972, making a Palm database containing a timestamp around
/// this time period extremely unlikely to occur naturally (I mean, bit flipping could happen?), so
/// this is a good enough measure of what timestamp format we're using.
///
/// This is the same heuristic that [palmdump](https://www.fourmilab.ch/palm/palmdump), and many
/// other Palm OS utilites, use for determining the timestamp format.
pub fn is_palm_epoch(timestamp: u32) -> bool {
	timestamp & (1 << 31) != 0
}

/// Convert an "old Palm epoch" timestamp to a UNIX epoch timestamp
pub fn palm_ts_to_unix_ts(timestamp: u32) -> i32 {
	i32::try_from(timestamp.wrapping_sub(SECONDS_BETWEEN_PALM_EPOCHS))
		.expect("integer overflow during timestamp conversion")
}

/// Convert a UNIX epoch timestamp to an "old Palm epoch" timestamp
pub fn unix_ts_to_palm_ts(timestamp: i32) -> u32 {
	(timestamp as u32).wrapping_add(SECONDS_BETWEEN_PALM_EPOCHS)
}

#[cfg(test)]
mod tests {
	use test_env_log::test;

	use super::*;

	#[test]
	fn is_palm_epoch_detects_unix() {
		assert_eq!(is_palm_epoch(0), false);
		assert_eq!(is_palm_epoch(0x613f3997), false);
	}

	#[test]
	fn is_palm_epoch_detects_palm() {
		assert_eq!(is_palm_epoch(0xb85898b0), true);
		assert_eq!(is_palm_epoch(0xdd64ea17), true);
	}

	#[test]
	fn unix_to_palm_to_unix() {
		assert_eq!(
			1009969200,
			palm_ts_to_unix_ts(unix_ts_to_palm_ts(1009969200))
		);
	}

	#[test]
	fn palm_to_unix_to_palm() {
		assert_eq!(
			3092855400,
			unix_ts_to_palm_ts(palm_ts_to_unix_ts(3092855400))
		);
	}

	#[test]
	fn palmtimestamp_decodes_palm_epoch() {
		let ts = PalmTimestamp(3092855400);
		assert_eq!(ts.as_unix_ts(), 1009969200);
	}

	#[test]
	fn palmtimestamp_passthru_unix_epoch() {
		let ts = PalmTimestamp(1009969200);
		assert_eq!(ts.as_unix_ts(), 1009969200);
	}
}
