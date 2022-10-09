//! `.nds` and `.narc` Fat + Fnt implementation

// Features

// Modules
pub mod dir;
pub mod fat;
pub mod file_ptr;
pub mod fnt;

// Exports
pub use self::{
	dir::{Dir, DirEntry, DirEntryKind},
	fat::FileAllocationTable,
	file_ptr::FilePtr,
	fnt::FileNameTable,
};
