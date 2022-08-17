//! File allocation table

// Modules
mod error;

// Exports
pub use self::error::FromReaderError;

// Imports
use {
	crate::FilePtr,
	itertools::Itertools,
	std::{io, iter},
};

/// File allocation table
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FileAllocationTable {
	/// All pointers
	pub ptrs: Vec<FilePtr>,
}

impl FileAllocationTable {
	/// Creates a file allocation table from a reader
	pub fn from_reader<R: io::Read>(reader: &mut R) -> Result<Self, FromReaderError> {
		let ptrs = iter::from_fn(move || {
			let mut bytes = [0; 8];
			match reader.read_exact(&mut bytes) {
				Ok(()) => Some(Ok(FilePtr::from_bytes(&bytes))),
				Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => None,
				Err(err) => Some(Err(err)),
			}
		})
		.try_collect::<_, Vec<_>, _>()
		.map_err(FromReaderError::ReadFile)?;

		Ok(Self { ptrs })
	}
}
