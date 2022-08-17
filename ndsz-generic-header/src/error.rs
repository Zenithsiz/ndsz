//! Errors

/// Error for [`Header::from_bytes`](super::Header::from_bytes)
#[derive(PartialEq, Eq, Clone, Debug, thiserror::Error)]
pub enum FromBytesError {
	/// Wrong constant
	#[error("Wrong constant: {constant:#x}")]
	WrongConstant { constant: u32 },

	/// Wrong header size
	#[error("Wrong header size: {header_size:#x}")]
	WrongHeaderSize { header_size: u16 },
}
