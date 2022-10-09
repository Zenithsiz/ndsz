//! Errors

// Imports

/// Error for [`Header::from_bytes`](super::Header::from_bytes)
#[derive(PartialEq, Eq, Clone, Debug, thiserror::Error)]
pub enum FromBytesError {
	/// Wrong chunk name
	#[error("Wrong chunk name {chunk_name:x?}, expected 'NARC'")]
	WrongChunkName { chunk_name: [u8; 4] },

	/// Wrong byte order
	#[error("Wrong byte order {byte_order:#x}, expected 0xfffe")]
	WrongByteOrder { byte_order: u16 },

	/// Unknown version
	#[error("Unknown version {version:#x}, expected 0x0100")]
	UnknownVersion { version: u16 },

	/// Wrong chunk size
	#[error("Wrong chunk size {chunk_size:#x}, expected 0x0010")]
	WrongChunkSize { chunk_size: u16 },

	/// Wrong chunks length
	#[error("Wrong number of chunks {chunks_len}, expected 3")]
	WrongChunksLen { chunks_len: u16 },
}
