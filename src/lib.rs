//! A collection of libraries and command-line utilities for interacting with Palm OS devices, and
//! the data stored on them.
//!
//! This crate, `palmrs`, re-exports all the major palm.rs library subcrates (with the `palmrs_`
//! prefix stripped from their names), as well as containing the command-line utilities provided by
//! palm.rs.
//!
//! This crate is not intended to be dependended on by other crates - instead, crates should depend
//! on the individual palm.rs subcrates they require - it exists solely to make installation of the
//! command-line tools via Cargo easier.
//!
//! For an up-to-date list of the available palm.rs subcrates and command-line tools;
//! documentation; and the issue tracker, see <https://github.com/u1f408/palmrs>.

pub use palmrs_database as database;
