//! Sub table entry

// Modules
mod error;

// Exports
pub use error::FromReaderError;

// Imports
use byteorder::{LittleEndian, ReadBytesExt};
use std::io;
use zutil::AsciiStrArr;

/// Sub table entry
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SubTableEntry {
	/// Name
	pub name: AsciiStrArr<0x80>,

	/// Kind
	pub kind: SubTableEntryKind,
}

/// Sub table entry kind
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SubTableEntryKind {
	File,
	Dir { id: u16 },
}


impl SubTableEntry {
	/// Reads a sub table entry from a reader
	pub fn from_reader<R: io::Read>(reader: &mut R) -> Result<Option<Self>, FromReaderError> {
		/// Reads the name from the reader
		fn read_name<R: io::Read>(reader: &mut R, len: u8) -> Result<AsciiStrArr<0x80>, FromReaderError> {
			let len = usize::from(len);

			let mut bytes = [0; 0x80];
			reader
				.read_exact(&mut bytes[..len])
				.map_err(FromReaderError::ReadName)?;

			AsciiStrArr::from_bytes(&bytes[..len]).map_err(FromReaderError::ParseName)
		}

		// Read the type / len
		let ty_len = reader.read_u8().map_err(FromReaderError::ReadTypeLen)?;
		let (name, kind) = match ty_len {
			0x0 => return Ok(None),
			len @ 0x1..=0x7f => (read_name(reader, len)?, SubTableEntryKind::File),
			len_0x80 @ 0x81..=0xff => (read_name(reader, len_0x80 - 0x80)?, SubTableEntryKind::Dir {
				id: reader.read_u16::<LittleEndian>().map_err(FromReaderError::ReadDirId)?,
			}),
			0x80 => return Err(FromReaderError::ReservedDirKind),
		};

		Ok(Some(Self { name, kind }))
	}
}
