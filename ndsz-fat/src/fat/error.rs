//! Errors

// Imports
use std::io;

/// Error for [`FileAllocationTable::from_reader`](super::FileAllocationTable::from_reader)
#[derive(Debug, thiserror::Error)]
pub enum FromReaderError {
	/// Unable to read file
	#[error("Unable to read file")]
	ReadFile(#[source] io::Error),
}
