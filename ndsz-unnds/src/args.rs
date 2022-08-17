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
}
