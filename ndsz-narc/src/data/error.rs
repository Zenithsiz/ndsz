//! Errors

// Imports
use std::io;

/// Error for [`Data::from_reader`](super::Data::from_reader)
#[derive(Debug, thiserror::Error)]
pub enum FromReaderError {
	/// Unable to read header
	#[error("Unable to read header")]
	ReadHeader(#[source] io::Error),

	/// Wrong header
	#[error("Wrong header")]
	WrongHeader,

	/// Unable to seek to slice reader to data
	#[error("Unable to slice reader to data")]
	SliceReader(#[source] io::Error),
}

/// Error for [`Data::narcless_from_reader`](super::Data::narcless_from_reader)
#[derive(Debug, thiserror::Error)]
pub enum NarclessFromReaderError {
	/// Unable to get data start pos
	#[error("Unable to get data start pos")]
	StartPos(#[source] io::Error),

	/// Unable to seek to slice reader to data
	#[error("Unable to slice reader to data")]
	SliceReader(#[source] io::Error),
}
