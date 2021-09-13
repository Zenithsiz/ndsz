//! Generic header for most nds file formats
//!
//! Adapted from `https://loveemu.hatenablog.com/entry/20091002/nds_formats`,
//! which itself is adapted from `http://llref.emutalk.net/nds_formats.htm` (dead link)

// Modules
mod error;

// Exports
pub use error::FromBytesError;

// Imports
use byteorder::{ByteOrder, LittleEndian};

/// Header
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Header {
	/// Magic
	pub magic: [u8; 4],

	/// Section size
	pub section_size: u32,

	/// Number of sub-sections
	pub sub_sections_len: u16,
}

impl Header {
	/// Parses a header from bytes
	pub fn from_bytes(bytes: &[u8; 0x10]) -> Result<Self, FromBytesError> {
		let bytes = zutil::array_split!(bytes,
			magic           : [0x4],
			constant        : [0x4],
			section_size    : [0x4],
			header_size     : [0x2],
			sub_sections_len: [0x2],
		);

		let constant = LittleEndian::read_u32(bytes.constant);
		let header_size = LittleEndian::read_u16(bytes.header_size);
		if constant != 0xfffe0001 {
			return Err(FromBytesError::WrongConstant { constant });
		}
		if header_size != 0x10 {
			return Err(FromBytesError::WrongHeaderSize { header_size });
		}


		Ok(Self {
			magic:            *bytes.magic,
			section_size:     LittleEndian::read_u32(bytes.section_size),
			sub_sections_len: LittleEndian::read_u16(bytes.sub_sections_len),
		})
	}
}
