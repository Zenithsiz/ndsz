//! Header

// Modules
mod error;

// Exports
pub use error::FromBytesError;

// Imports
use byteorder::{ByteOrder, LittleEndian};

/// Header
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Header {
	/// File size
	pub file_size: u32,
}

impl Header {
	/// Parses a header data from bytes
	pub fn from_bytes(bytes: &[u8; 0x10]) -> Result<Self, FromBytesError> {
		let bytes = zutil::array_split!(bytes,
			chunk_name: [0x4],
			byte_order: [0x2],
			version   : [0x2],
			file_size : [0x4],
			chunk_size: [0x2],
			chunks_len: [0x2],
		);

		let chunk_name = *bytes.chunk_name;
		let byte_order = LittleEndian::read_u16(bytes.byte_order);
		let version = LittleEndian::read_u16(bytes.version);
		let chunk_size = LittleEndian::read_u16(bytes.chunk_size);
		let chunks_len = LittleEndian::read_u16(bytes.chunks_len);
		if chunk_name != *b"NARC" {
			return Err(FromBytesError::WrongChunkName { chunk_name });
		}
		if byte_order != 0xfffe {
			return Err(FromBytesError::WrongByteOrder { byte_order });
		}
		if version != 0x0100 {
			return Err(FromBytesError::UnknownVersion { version });
		}
		if chunk_size != 0x0010 {
			return Err(FromBytesError::WrongChunkSize { chunk_size });
		}
		if chunks_len != 3 {
			return Err(FromBytesError::WrongChunksLen { chunks_len });
		}

		Ok(Self {
			file_size: LittleEndian::read_u32(bytes.file_size),
		})
	}
}
