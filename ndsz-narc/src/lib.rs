//! `.narc` implementation

// Features
#![feature()]

// Modules
pub mod data;
mod error;
pub mod fat_header;
pub mod fnt_header;
pub mod header;

// Exports
pub use data::Data;
pub use error::{FromReaderError, NarclessFromReaderError};
pub use fat_header::FatHeader;
pub use fnt_header::FntHeader;
pub use header::Header;

// Imports
use byteorder::{LittleEndian, ReadBytesExt};
use ndsz_fat::{FileAllocationTable, FileNameTable};
use std::io::{self, Read, Seek, SeekFrom};
use zutil::{IoSlice, ReadByteArray};

/// Narc file
#[derive(Clone, Debug)]
pub struct Narc<R> {
	/// Fat
	pub fat: FileAllocationTable,

	/// Data
	pub fnt: FileNameTable,

	/// File data
	pub data: Data<R>,
}

impl<R> Narc<R> {
	/// Reads the narc from a reader
	pub fn from_reader(mut reader: R) -> Result<Self, FromReaderError>
	where
		R: io::Read + io::Seek,
	{
		// Try to read the header
		let header = {
			let bytes = reader.read_byte_array().map_err(FromReaderError::ReadHeader)?;
			Header::from_bytes(&bytes).map_err(FromReaderError::ParseHeader)?
		};

		// Limit the reader to the file size
		// Note: Technically not required, but will catch faulty narcs
		// TODO: Avoid doing the 2 seeks here
		let mut reader = IoSlice::new(reader, ..u64::from(header.file_size)).map_err(FromReaderError::SliceReader)?;
		reader.seek(SeekFrom::Current(0x10)).map_err(FromReaderError::SeekFat)?;

		// Read the fat
		let fat = {
			// Read the header
			let header_bytes = reader.read_byte_array().map_err(FromReaderError::ReadFatHeader)?;
			let header = FatHeader::from_bytes(&header_bytes).ok_or(FromReaderError::WrongFatHeader)?;

			// And then the fat
			let fat_len = header.chunk_size - FatHeader::SIZE as u32;
			FileAllocationTable::from_reader(&mut reader.by_ref().take(u64::from(fat_len)))
				.map_err(FromReaderError::ReadFat)?
		};

		// Then the fnt
		let fnt = {
			// Read the header
			let header_bytes = reader.read_byte_array().map_err(FromReaderError::ReadFntHeader)?;
			let header = FntHeader::from_bytes(&header_bytes).ok_or(FromReaderError::WrongFntHeader)?;

			// And then the fnt
			let fnt_len = header.chunk_size - FntHeader::SIZE as u32;
			let cur_reader_pos = reader.stream_position().map_err(FromReaderError::FntStartPos)?;
			let mut fnt_slice =
				IoSlice::new_take(reader.by_ref(), u64::from(fnt_len)).map_err(FromReaderError::SliceFnt)?;
			let fnt = FileNameTable::from_reader(&mut fnt_slice).map_err(FromReaderError::ReadFnt)?;

			// After seek to the end of the fnt
			reader
				.seek(io::SeekFrom::Start(cur_reader_pos + u64::from(fnt_len)))
				.map_err(FromReaderError::FntSeekEnd)?;

			fnt
		};

		// And the data
		let data = Data::from_reader(reader.into_inner()).map_err(FromReaderError::ReadData)?;

		Ok(Self { fat, fnt, data })
	}

	/// Reads a narcless variant from a reader
	pub fn narcless_from_reader(mut reader: R) -> Result<Self, NarclessFromReaderError>
	where
		R: io::Read + io::Seek,
	{
		// Read the header
		let fnt_offset = reader
			.read_u32::<LittleEndian>()
			.map_err(NarclessFromReaderError::ReadFntOffset)?;
		let fnt_len = reader
			.read_u32::<LittleEndian>()
			.map_err(NarclessFromReaderError::ReadFntLen)?;
		let fat_offset = reader
			.read_u32::<LittleEndian>()
			.map_err(NarclessFromReaderError::ReadFatOffset)?;
		let fat_len = reader
			.read_u32::<LittleEndian>()
			.map_err(NarclessFromReaderError::ReadFatLen)?;

		// Read the fnt
		let fnt = {
			let mut fnt_slice =
				IoSlice::new_with_offset_len(reader.by_ref(), u64::from(fnt_offset), u64::from(fnt_len))
					.map_err(NarclessFromReaderError::FntSlice)?;
			FileNameTable::from_reader(&mut fnt_slice).map_err(NarclessFromReaderError::ReadFnt)?
		};

		// Then the fat
		let fat = {
			let mut fat_slice =
				IoSlice::new_with_offset_len(reader.by_ref(), u64::from(fat_offset), u64::from(fat_len))
					.map_err(NarclessFromReaderError::FatSlice)?;
			FileAllocationTable::from_reader(&mut fat_slice).map_err(NarclessFromReaderError::ReadFat)?
		};

		// And the data
		reader
			.seek(io::SeekFrom::Start(u64::from(fat_offset + fat_len)))
			.map_err(NarclessFromReaderError::SeekData)?;
		let data = Data::narcless_from_reader(reader).map_err(NarclessFromReaderError::ReadData)?;

		Ok(Self { fat, fnt, data })
	}
}
