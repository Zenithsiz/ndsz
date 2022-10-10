//! Arguments

// Imports
use std::path::PathBuf;

/// Arguments
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(clap::Parser)]
pub struct Args {
	/// Input path
	pub input_path: PathBuf,

	/// Output path.
	///
	/// Defaults to `input_path` without an extension
	#[clap(long = "output", short = 'o')]
	pub output_path: Option<PathBuf>,

	/// Fat output
	///
	/// Outputs the fat as a yaml file for debugging
	#[clap(long = "fat")]
	pub fat: bool,

	/// Fnt output
	///
	/// Outputs the fnt as a yaml file for debugging
	#[clap(long = "fnt")]
	pub fnt: bool,

	/// Extract fat files
	///
	/// Extracts all fat files independently of the fnt
	#[clap(long = "fat-files")]
	pub fat_files: bool,
}
