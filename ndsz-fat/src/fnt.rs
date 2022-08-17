//! File name table

// Modules
mod error;
pub mod main_table;
pub mod sub_table;

// Exports
pub use self::{
	error::FromReaderError,
	main_table::{MainTable, MainTableEntry},
	sub_table::{SubTable, SubTableEntry, SubTableEntryKind},
};

// Imports
use {crate::Dir, std::io};

/// File name table
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FileNameTable {
	/// Root directory
	pub root: Dir,
}

impl FileNameTable {
	/// Reads the FNT from a reader
	pub fn from_reader<R: io::Read + io::Seek>(reader: &mut R) -> Result<Self, FromReaderError> {
		// Read the main table
		let main_table = MainTable::from_reader(reader).map_err(FromReaderError::ReadMainTable)?;

		// Read the root entry
		let root = main_table
			.root_entry
			.read_dir(reader, 61440, &main_table.entries)
			.map_err(FromReaderError::ReadRootDir)?;

		Ok(Self { root })
	}
}
