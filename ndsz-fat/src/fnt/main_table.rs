//! Fnt main table

// Modules
mod error;

// Exports
pub use self::error::{FromReaderError, ReadDirError, ReadSubTableError};

// Imports
use {
	super::{SubTable, SubTableEntryKind},
	crate::{Dir, DirEntry, DirEntryKind},
	byteorder::{ByteOrder, LittleEndian},
	itertools::Itertools,
	std::{io, iter},
	zutil::{IoSlice, ReadByteArray},
};

/// Main table
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct MainTable {
	/// Root entry
	pub root_entry: MainTableEntry,

	/// All entries
	pub entries: Vec<MainTableEntry>,
}

impl MainTable {
	/// Reads the main table from a reader
	pub fn from_reader<R: io::Read>(reader: &mut R) -> Result<Self, FromReaderError> {
		// Read the root entry
		let root_entry = {
			let bytes = reader.read_byte_array().map_err(FromReaderError::ReadRootEntry)?;
			MainTableEntry::from_bytes(&bytes)
		};

		// Note: On the root entry, since there is no parent, the total number of entries is
		//       encoded instead.
		let entries_len = root_entry.parent_id;

		// Then read all other entries
		let entries = iter::from_fn(move || match reader.read_byte_array() {
			Ok(bytes) => Some(Ok(MainTableEntry::from_bytes(&bytes))),
			Err(err) => Some(Err(err)),
		})
		.take(usize::from(entries_len) - 1) // Note: `-1` because we already read the root.
		.try_collect::<_, Vec<_>, _>()
		.map_err(FromReaderError::ReadEntry)?;

		Ok(Self { root_entry, entries })
	}
}

/// Main table entry
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct MainTableEntry {
	/// Offset to sub table
	pub sub_table_offset: u32,

	/// First file id
	pub first_file_id: u16,

	/// Parent id
	// Note: Used as the total number of directories in the root entry
	pub parent_id: u16,
}

impl MainTableEntry {
	/// Parses an entry from bytes
	pub fn from_bytes(bytes: &[u8; 0x8]) -> Self {
		let bytes = ndsz_bytes::array_split!(bytes,
			sub_table_offset: [0x4],
			first_file_id   : [0x2],
			parent_id       : [0x2],
		);

		Self {
			sub_table_offset: LittleEndian::read_u32(bytes.sub_table_offset),
			first_file_id:    LittleEndian::read_u16(bytes.first_file_id),
			parent_id:        LittleEndian::read_u16(bytes.parent_id),
		}
	}

	/// Reads the sub-table from this entry
	pub fn read_sub_table<R: io::Read + io::Seek>(&self, reader: &mut R) -> Result<SubTable, ReadSubTableError> {
		let mut slice =
			IoSlice::new(reader, u64::from(self.sub_table_offset)..).map_err(ReadSubTableError::CreateSlice)?;

		SubTable::from_reader(&mut slice).map_err(ReadSubTableError::ReadSubTable)
	}

	/// Reads a directory from this entry
	pub fn read_dir<R: io::Read + io::Seek>(
		&self,
		reader: &mut R,
		id: u16,
		main_entries: &[MainTableEntry],
	) -> Result<Dir, ReadDirError> {
		let sub_table = self.read_sub_table(reader).map_err(ReadDirError::ReadSubTable)?;
		let mut parent_main_entries = main_entries.iter().filter(|main_entry| main_entry.parent_id == id);
		let mut cur_file_id = self.first_file_id;

		let entries = sub_table
			.entries
			.iter()
			.map(|&sub_entry| {
				let kind = match sub_entry.kind {
					SubTableEntryKind::File => {
						let id = cur_file_id;
						cur_file_id += 1;
						DirEntryKind::File { id }
					},
					SubTableEntryKind::Dir { id } => {
						let main_entry = parent_main_entries.next().ok_or(ReadDirError::NoMainEntry)?;
						let dir = main_entry
							.read_dir(reader, id, main_entries)
							.map_err(|err| ReadDirError::ReadSubDir(Box::new(err)))?;

						DirEntryKind::Dir { id, dir }
					},
				};

				Ok(DirEntry {
					name: sub_entry.name,
					kind,
				})
			})
			.try_collect::<_, Vec<_>, _>()?;

		Ok(Dir { id, entries })
	}
}
