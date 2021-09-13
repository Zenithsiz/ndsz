//! Errors

// Imports
use crate::fnt::sub_table;
use std::io;

/// Error for [`MainTable::from_reader`](super::MainTable::from_reader)
#[derive(Debug, thiserror::Error)]
pub enum FromReaderError {
	/// Unable to read root entry
	#[error("Unable to read root entry")]
	ReadRootEntry(#[source] io::Error),

	/// Unable to read entry
	#[error("Unable to read entry")]
	ReadEntry(#[source] io::Error),
}


/// Error for [`MainTableEntry::read_sub_table`](super::MainTableEntry::read_sub_table)
#[derive(Debug, thiserror::Error)]
pub enum ReadSubTableError {
	/// Unable to create sub-table slice
	#[error("Unable to create sub-table slice")]
	CreateSlice(#[source] io::Error),

	/// Unable to read sub-table
	#[error("Unable to read sub-table")]
	ReadSubTable(#[source] sub_table::FromReaderError),
}


/// Error for [`MainTableEntry::read_dir`](super::MainTableEntry::read_dir)
#[derive(Debug, thiserror::Error)]
pub enum ReadDirError {
	/// Unable to read sub-table
	#[error("Unable to read sub-table")]
	ReadSubTable(#[source] ReadSubTableError),

	/// Directory had no main entry
	#[error("Directory had no main entry")]
	NoMainEntry,

	/// Unable to read sub-directory
	#[error("Unable to read sub-directory")]
	ReadSubDir(#[source] Box<Self>),
}
