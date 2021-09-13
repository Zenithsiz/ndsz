//! Arguments

// Imports
use std::path::PathBuf;

/// Arguments
#[derive(PartialEq, Clone, Debug)]
pub struct Args {
	/// Game path
	pub game_path: PathBuf,

	/// Output path
	pub output_path: PathBuf,
}

/// Retrieves the arguments
pub fn get() -> Result<Args, anyhow::Error> {
	const GAME_PATH_ARG: &str = "game-path";
	const OUTPUT_PATH_ARG: &str = "output-path";

	// Get matches
	let matches = clap::App::new("ndsz-unnds")
		.help("`.nds` extractor")
		.arg(clap::Arg::with_name(GAME_PATH_ARG).long("game").index(1).required(true))
		.arg(
			clap::Arg::with_name(OUTPUT_PATH_ARG)
				.long("output")
				.index(2)
				.required(true),
		)
		.get_matches();

	// Get the game path
	let game_path = matches
		.value_of_os(GAME_PATH_ARG)
		.map(PathBuf::from)
		.expect("Required argument missing");

	// And the game path
	let output_path = matches
		.value_of_os(OUTPUT_PATH_ARG)
		.map(PathBuf::from)
		.expect("Required argument missing");

	Ok(Args { game_path, output_path })
}
