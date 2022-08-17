//! Errors

// Imports

/// Error for [`Header::from_bytes`](super::Header::from_bytes)
#[derive(PartialEq, Eq, Clone, Debug, thiserror::Error)]
pub enum FromBytesError {
	/// Wrong chunk name
	#[error("Wrong chunk name: {chunk_name:x?}")]
	WrongChunkName { chunk_name: [u8; 4] },

	/// Wrong byte order
	#[error("Wrong byte order: {byte_order:#x}")]
	WrongByteOrder { byte_order: u16 },

	/// Unknown version
	#[error("Unknown version: {version}")]
	UnknownVersion { version: u16 },

	/// Wrong chunk size
	#[error("Wrong chunk size: {chunk_size:#x}")]
	WrongChunkSize { chunk_size: u16 },

	/// Wrong chunks length
	#[error("Wrong number of chunks: {chunks_len}")]
	WrongChunksLen { chunks_len: u16 },
}
