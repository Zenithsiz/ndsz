//! Arguments

// Imports
use std::path::PathBuf;

/// Arguments
#[derive(PartialEq, Clone, Debug)]
pub struct Args {
	/// Input path
	pub input_path: PathBuf,

	/// Output path
	pub output_path: PathBuf,

	/// Narcless
	pub narcless: bool,
}

/// Retrieves the arguments
pub fn get() -> Result<Args, anyhow::Error> {
	const INPUT_PATH_ARG: &str = "input-path";
	const OUTPUT_PATH_ARG: &str = "output-path";
	const NARCLESS_ARG: &str = "narcless";

	// Get matches
	let matches = clap::App::new("ndsz-unnarc")
		.help("`.narc` extractor")
		.arg(
			clap::Arg::with_name(INPUT_PATH_ARG)
				.long("input")
				.index(1)
				.required(true),
		)
		.arg(
			clap::Arg::with_name(OUTPUT_PATH_ARG)
				.long("output")
				.index(2)
				.required(true),
		)
		.arg(clap::Arg::with_name(NARCLESS_ARG).long("narcless"))
		.get_matches();

	// Get the input path
	let input_path = matches
		.value_of_os(INPUT_PATH_ARG)
		.map(PathBuf::from)
		.expect("Required argument missing");

	// The output path
	let output_path = matches
		.value_of_os(OUTPUT_PATH_ARG)
		.map(PathBuf::from)
		.expect("Required argument missing");

	// And if to decode narcless
	let narcless = matches.is_present(NARCLESS_ARG);

	Ok(Args {
		input_path,
		output_path,
		narcless,
	})
}
