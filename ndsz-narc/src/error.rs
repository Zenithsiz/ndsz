//! Errors

// Imports
use crate::{data, header};
use ndsz_fat::{fat, fnt};
use std::io;

/// Error for [`Narc::from_reader`](super::Narc::from_reader)
#[derive(Debug, thiserror::Error)]
pub enum FromReaderError {
	/// Unable to read header
	#[error("Unable to read header")]
	ReadHeader(#[source] io::Error),

	/// Unable to parse header
	#[error("Unable to parse header")]
	ParseHeader(#[source] header::FromBytesError),

	/// Unable to slice reader to file size
	#[error("Unable to slice reader to file size")]
	SliceReader(#[source] io::Error),

	/// Unable to read fat header
	#[error("Unable to read fat header")]
	ReadFatHeader(#[source] io::Error),

	/// Wrong fat header
	#[error("Wrong fat header")]
	WrongFatHeader,

	/// Unable to read fat
	#[error("Unable to read fat")]
	ReadFat(#[source] fat::FromReaderError),

	/// Unable to read fat header
	#[error("Unable to read fnt header")]
	ReadFntHeader(#[source] io::Error),

	/// Wrong fnt header
	#[error("Wrong fnt header")]
	WrongFntHeader,

	/// Unable to get fnt start pos
	#[error("Unable to get fnt start pos")]
	FntStartPos(#[source] io::Error),

	/// Unable to slice reader fnt
	#[error("Unable to slice reader fnt")]
	SliceFnt(#[source] io::Error),

	/// Unable to read fnt
	#[error("Unable to read fnt")]
	ReadFnt(#[source] fnt::FromReaderError),

	/// Unable to seek to fnt end
	#[error("Unable to seek to fnt end")]
	FntSeekEnd(#[source] io::Error),

	/// Unable to read data
	#[error("Unable to read data")]
	ReadData(#[source] data::FromReaderError),
}

/// Error for [`Narc::narcless_from_reader`](super::Narc::narcless_from_reader)
#[derive(Debug, thiserror::Error)]
pub enum NarclessFromReaderError {
	/// Unable to read fnt offset
	#[error("Unable to read fnt offset")]
	ReadFntOffset(#[source] io::Error),

	/// Unable to read fnt len
	#[error("Unable to read fnt len")]
	ReadFntLen(#[source] io::Error),

	/// Unable to read fat offset
	#[error("Unable to read fat offset")]
	ReadFatOffset(#[source] io::Error),

	/// Unable to read fat len
	#[error("Unable to read fat len")]
	ReadFatLen(#[source] io::Error),

	/// Unable to create fnt slice
	#[error("Unable to create fnt slice")]
	FntSlice(#[source] io::Error),

	/// Unable to create fat slice
	#[error("Unable to create fat slice")]
	FatSlice(#[source] io::Error),

	/// Unable to read fnt
	#[error("Unable to read fnt")]
	ReadFnt(#[source] fnt::FromReaderError),

	/// Unable to read fat
	#[error("Unable to read fat")]
	ReadFat(#[source] fat::FromReaderError),

	/// Unable to seek to data
	#[error("Unable to seek to data")]
	SeekData(#[source] io::Error),

	/// Unable to read data
	#[error("Unable to read data")]
	ReadData(#[source] data::NarclessFromReaderError),
}
