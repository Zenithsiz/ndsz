//! Fat debugging

// Imports
use {
	anyhow::Context,
	ndsz_fat::FileAllocationTable,
	std::{collections::BTreeMap, fs, path::PathBuf},
};

/// Outputs `fat` as yaml to `path`
pub fn output_fat_yaml(fat: &FileAllocationTable, path: PathBuf) -> Result<(), anyhow::Error> {
	let fat = FatYaml {
		files: fat
			.ptrs
			.iter()
			.map(|ptr| FatFileYaml {
				start: ptr.start_address,
				end:   ptr.end_address,
			})
			.enumerate()
			.collect(),
	};

	let output = fs::File::create(&path).with_context(|| format!("Unable to create file {path:?}"))?;
	serde_yaml::to_writer(output, &fat).context("Unable to serialize fat")?;

	Ok(())
}

/// Fat
#[derive(Clone, Debug)]
#[derive(serde::Serialize)]
struct FatYaml {
	/// All files, by inode
	pub files: BTreeMap<usize, FatFileYaml>,
}

/// Fat file
#[derive(Clone, Debug)]
#[derive(serde::Serialize)]
struct FatFileYaml {
	/// Start address
	pub start: u32,

	/// End address
	pub end: u32,
}
