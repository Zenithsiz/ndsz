//! Errors

// Imports
use super::main_table;

/// Error for [`FileNameTable::from_reader`](super::FileNameTable::from_reader)
#[derive(Debug, thiserror::Error)]
pub enum FromReaderError {
	/// Unable to read main table
	#[error("Unable to read main table")]
	ReadMainTable(#[source] main_table::FromReaderError),

	/// Unable to read root directory
	#[error("Unable to read root directory")]
	ReadRootDir(#[source] main_table::ReadDirError),
}
