//! Arguments

// Imports
use std::path::PathBuf;

/// Arguments
#[derive(PartialEq, Eq, Clone, Debug)]
#[derive(clap::Parser)]
pub struct Args {
	/// Input path
	pub input_path: PathBuf,

	/// Output path
	///
	/// Defaults to `input-path` without an extension
	#[clap(long = "output", short = 'o')]
	pub output_path: Option<PathBuf>,

	/// Narcless
	#[clap(long = "narcless")]
	pub narcless: bool,

	/// Extract fat on empty fnt
	#[clap(long = "extract-fat-on-empty-fnt")]
	pub extract_fat_on_empty_fnt: bool,
}
