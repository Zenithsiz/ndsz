//! Fnt header

// Imports
use byteorder::{ByteOrder, LittleEndian};

/// Narc fnt header
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct FntHeader {
	/// Chunk size
	pub chunk_size: u32,
}

impl FntHeader {
	/// Header size
	pub const SIZE: usize = 0x8;

	/// Parses a header data from bytes
	pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Option<Self> {
		let bytes = ndsz_bytes::array_split!(bytes,
			chunk_name: [0x4],
			chunk_size: [0x4],
		);

		if bytes.chunk_name != b"BTNF" {
			return None;
		}

		Some(Self {
			chunk_size: LittleEndian::read_u32(bytes.chunk_size),
		})
	}
}
