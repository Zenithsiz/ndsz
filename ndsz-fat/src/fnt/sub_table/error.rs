//! Errors

// Imports
use super::entry;

/// Error for [`SubTable::from_reader`](super::SubTable::from_reader)
#[derive(Debug, thiserror::Error)]
pub enum FromReaderError {
	/// Unable to read entry
	#[error("Unable to read entry")]
	ReadEntry(#[source] entry::FromReaderError),
}
