//! Fat header

// Imports
use byteorder::{ByteOrder, LittleEndian};

/// Narc fat header
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct FatHeader {
	/// Chunk size
	pub chunk_size: u32,

	/// Number of files
	pub files_len: u16,

	/// Reserved
	pub reserved: u16,
}

impl FatHeader {
	/// Header size
	pub const SIZE: usize = 0xc;

	/// Parses a header data from bytes
	pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Option<Self> {
		let bytes = zutil::array_split!(bytes,
			chunk_name: [0x4],
			chunk_size: [0x4],
			files_len : [0x2],
			reserved  : [0x2],
		);

		if bytes.chunk_name != b"BTAF" {
			return None;
		}

		Some(Self {
			chunk_size: LittleEndian::read_u32(bytes.chunk_size),
			files_len:  LittleEndian::read_u16(bytes.files_len),
			reserved:   LittleEndian::read_u16(bytes.reserved),
		})
	}
}
