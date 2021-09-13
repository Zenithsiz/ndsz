//! Errors

// Imports
use zutil::ascii_str_arr;

/// Error for [`Header::from_bytes`](super::Header::from_bytes)
#[derive(PartialEq, Clone, Debug, thiserror::Error)]
pub enum FromBytesError {
	/// Unable to read game title
	#[error("Unable to read game title")]
	GameTitle(#[source] ascii_str_arr::FromBytesError<0xc>),

	/// Unable to read game code
	#[error("Unable to read game code")]
	GameCode(#[source] ascii_str_arr::FromBytesError<0x4>),

	/// Unable to read maker code
	#[error("Unable to read maker code")]
	MakerCode(#[source] ascii_str_arr::FromBytesError<0x2>),

	/// Unable to read unit code
	#[error("Unable to read unit code")]
	UnitCode,
}
