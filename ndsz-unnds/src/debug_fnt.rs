//! Fnt debugging

// Imports
use {
	anyhow::Context,
	ndsz_fat::{Dir, DirEntry, DirEntryKind, FileNameTable},
	ndsz_util::AsciiStrArr,
	std::{fs, path::PathBuf},
};

/// Outputs `fnt` as yaml to `path`
pub fn output_fnt_yaml(fnt: &FileNameTable, path: PathBuf) -> Result<(), anyhow::Error> {
	let fnt = FntYaml::new(fnt);

	let output = fs::File::create(&path).with_context(|| format!("Unable to create file {path:?}"))?;
	serde_yaml::to_writer(output, &fnt).context("Unable to serialize fnt")?;

	Ok(())
}

/// Fnt
#[derive(Clone, Debug)]
#[derive(serde::Serialize)]
struct FntYaml {
	/// Root directory
	pub root: FntDirYaml,
}

impl FntYaml {
	fn new(fnt: &FileNameTable) -> Self {
		Self {
			root: FntDirYaml::new(&fnt.root),
		}
	}
}

/// Fnt directory
#[derive(Clone, Debug)]
#[derive(serde::Serialize)]
struct FntDirYaml {
	/// Entries
	pub entries: Vec<FntDirEntryYaml>,
}

impl FntDirYaml {
	pub fn new(dir: &Dir) -> Self {
		Self {
			entries: dir.entries.iter().map(FntDirEntryYaml::new).collect(),
		}
	}
}

/// Fnt directory entry
#[derive(Clone, Debug)]
#[derive(serde::Serialize)]
struct FntDirEntryYaml {
	/// Name
	pub name: AsciiStrArr<0x80>,

	/// Kind
	#[serde(flatten)]
	pub kind: FntDirEntryKindYaml,
}

impl FntDirEntryYaml {
	fn new(entry: &DirEntry) -> Self {
		Self {
			name: entry.name,
			kind: FntDirEntryKindYaml::new(&entry.kind),
		}
	}
}

/// Fnt directory entry kind
#[derive(Clone, Debug)]
#[derive(serde::Serialize)]
#[serde(untagged)]
enum FntDirEntryKindYaml {
	File { id: u16 },
	Dir { id: u16, dir: FntDirYaml },
}

impl FntDirEntryKindYaml {
	pub fn new(kind: &DirEntryKind) -> Self {
		match *kind {
			DirEntryKind::File { id } => Self::File { id },
			DirEntryKind::Dir { id, ref dir } => Self::Dir {
				id,
				dir: FntDirYaml::new(dir),
			},
		}
	}
}
