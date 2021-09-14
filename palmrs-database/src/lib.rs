//! Support for reading, and eventually writing, the Palm OS database formats (PRC and PDB)

mod format;
pub mod header;
pub mod record;
pub mod time;

pub use self::format::{DatabaseFormat, PalmDatabase, PdbDatabase, PrcDatabase};
