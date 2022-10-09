//! Data

// Modules
mod error;

// Exports
pub use error::{FromReaderError, NarclessFromReaderError};

// Imports
use {
	byteorder::{ByteOrder, LittleEndian},
	std::io,
	zutil::{IoSlice, ReadByteArray},
};

/// Narc data
#[derive(Clone, Debug)]
pub struct Data<R>(pub IoSlice<R>);

impl<R> Data<R> {
	/// Reads the narc data from a reader
	pub fn from_reader(mut reader: R) -> Result<Self, FromReaderError>
	where
		R: io::Read + io::Seek,
	{
		// Read the header
		let header = {
			let bytes = reader.read_byte_array().map_err(FromReaderError::ReadHeader)?;
			Header::from_bytes(&bytes).ok_or(FromReaderError::WrongHeader)?
		};

		// Then read the data
		let data_len = header.chunk_size - Header::SIZE as u32;
		let data = IoSlice::new_take(reader, u64::from(data_len)).map_err(FromReaderError::SliceReader)?;

		Ok(Self(data))
	}

	/// Reads the narc data from a narcless
	pub fn narcless_from_reader(mut reader: R) -> Result<Self, NarclessFromReaderError>
	where
		R: io::Read + io::Seek,
	{
		// Read the data
		let cur_pos = reader.stream_position().map_err(NarclessFromReaderError::StartPos)?;
		let data = IoSlice::new(reader, cur_pos..).map_err(NarclessFromReaderError::SliceReader)?;

		Ok(Self(data))
	}
}

/// Narc data header
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Header {
	/// Chunk size
	pub chunk_size: u32,
}

impl Header {
	/// Header size
	pub const SIZE: usize = 0x8;

	/// Parses a header data from bytes
	pub fn from_bytes(bytes: &[u8; Self::SIZE]) -> Option<Self> {
		let bytes = ndsz_bytes::array_split!(bytes,
			chunk_name: [0x4],
			chunk_size: [0x4],
		);

		if bytes.chunk_name != b"GMIF" {
			return None;
		}

		Some(Self {
			chunk_size: LittleEndian::read_u32(bytes.chunk_size),
		})
	}
}
