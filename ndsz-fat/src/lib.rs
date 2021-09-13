//! `.nds` and `.narc` Fat + Fnt implementation

// Features
#![feature(generic_associated_types)]

// Modules
pub mod dir;
pub mod fat;
pub mod file_ptr;
pub mod fnt;

// Exports
pub use dir::{Dir, DirEntry, DirEntryKind};
pub use fat::FileAllocationTable;
pub use file_ptr::FilePtr;
pub use fnt::FileNameTable;
