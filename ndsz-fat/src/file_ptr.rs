//! File pointer

// Imports
use byteorder::{ByteOrder, LittleEndian};

/// File pointer
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct FilePtr {
	/// Start address
	pub start_address: u32,

	/// End address
	pub end_address: u32,
}

impl FilePtr {
	/// Parses a header data from bytes
	pub fn from_bytes(bytes: &[u8; 0x8]) -> Self {
		let bytes = ndsz_bytes::array_split!(bytes,
			start_address: [0x4],
			  end_address: [0x4],
		);

		Self {
			start_address: LittleEndian::read_u32(bytes.start_address),
			end_address:   LittleEndian::read_u32(bytes.end_address),
		}
	}
}
