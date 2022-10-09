//! Errors

// Imports
use {ndsz_util::ascii_str_arr, std::io};

/// Error for [`SubTableEntry::from_reader`](super::SubTableEntry::from_reader)
#[derive(Debug, thiserror::Error)]
pub enum FromReaderError {
	/// Unable to read name
	#[error("Unable to read name")]
	ReadName(#[source] io::Error),

	/// Unable to parse name
	#[error("Unable to parse name")]
	ParseName(#[source] ascii_str_arr::FromBytesError<0x80>),

	/// Unable to read the type/len field
	#[error("Unable to read the type/len field")]
	ReadTypeLen(#[source] io::Error),

	/// Unable to read directory id
	#[error("Unable to read directory id")]
	ReadDirId(#[source] io::Error),

	/// Found a reserved dir kind
	#[error("Found a reserved dir kind")]
	ReservedDirKind,
}
