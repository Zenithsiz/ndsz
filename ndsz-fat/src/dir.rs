//! Directory

// Modules
pub mod visitor;

// Exports
pub use visitor::Visitor;

// Imports
use zutil::AsciiStrArr;

/// Directory
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Dir {
	/// Id
	pub id: u16,

	/// Entries
	pub entries: Vec<DirEntry>,
}

impl Dir {
	/// Walks through all entries in this directory and sub-directory.
	///
	/// Note: Performs a depth-first walk.
	pub fn walk<V: Visitor>(&self, visitor: &mut V) -> Result<(), V::Error> {
		for entry in &self.entries {
			match entry.kind {
				DirEntryKind::File { id } => visitor.visit_file(&entry.name, id)?,
				DirEntryKind::Dir { ref dir, id } => {
					let mut visitor = visitor.visit_dir(&entry.name, id)?;

					// Then recurse
					dir.walk(&mut visitor)?;
				},
			}
		}

		Ok(())
	}
}

/// Directory entry
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DirEntry {
	/// Name
	pub name: AsciiStrArr<0x80>,

	/// Kind
	pub kind: DirEntryKind,
}

/// Directory entry kind
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum DirEntryKind {
	/// File
	File {
		/// File id
		id: u16,
	},

	/// Dir
	Dir {
		/// Dir id
		id: u16,

		/// Dir
		dir: Dir,
	},
}
