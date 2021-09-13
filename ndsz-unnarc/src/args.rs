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

	/// Extract fat on empty fnt
	pub extract_fat_on_empty_fnt: bool,
}

/// Retrieves the arguments
pub fn get() -> Result<Args, anyhow::Error> {
	const INPUT_PATH_ARG: &str = "input-path";
	const OUTPUT_PATH_ARG: &str = "output-path";
	const NARCLESS_ARG: &str = "narcless";
	const EXTRACT_FAT_ON_EMPTY_FNT_ARG: &str = "extract-fat-on-empty-fnt";

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
		.arg(
			clap::Arg::with_name(EXTRACT_FAT_ON_EMPTY_FNT_ARG)
				.help("If the fat should be extracted if the fnt is empty")
				.long("extract-fat-on-empty-fnt"),
		)
		.get_matches();

	let input_path = matches
		.value_of_os(INPUT_PATH_ARG)
		.map(PathBuf::from)
		.expect("Required argument missing");
	let output_path = matches
		.value_of_os(OUTPUT_PATH_ARG)
		.map(PathBuf::from)
		.expect("Required argument missing");
	let narcless = matches.is_present(NARCLESS_ARG);
	let extract_fat_on_empty_fnt = matches.is_present(EXTRACT_FAT_ON_EMPTY_FNT_ARG);

	Ok(Args {
		input_path,
		output_path,
		narcless,
		extract_fat_on_empty_fnt,
	})
}
