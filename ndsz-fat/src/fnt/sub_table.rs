//! Fnt sub table

// Modules
pub mod entry;
mod error;

// Exports
pub use self::{
	entry::{SubTableEntry, SubTableEntryKind},
	error::FromReaderError,
};

// Imports
use {
	itertools::Itertools,
	std::{io, iter},
};

/// Sub table
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SubTable {
	/// All entries
	pub entries: Vec<SubTableEntry>,
}

impl SubTable {
	/// Reads a sub table from a reader
	pub fn from_reader<R: io::Read>(reader: &mut R) -> Result<Self, FromReaderError> {
		// Read all entries
		let entries = iter::from_fn(move || SubTableEntry::from_reader(reader).transpose())
			.try_collect()
			.map_err(FromReaderError::ReadEntry)?;

		Ok(Self { entries })
	}
}
