//! Unpacks a `.nds`

// Features
#![feature(fs_try_exists)]

// Modules
mod args;
mod debug_fat;
mod debug_fnt;
mod extract;

// Imports
use {
	self::{
		args::Args,
		debug_fat::output_fat_yaml,
		debug_fnt::output_fnt_yaml,
		extract::{extract_all_parts, extract_fat_dir, extract_fat_raw},
	},
	anyhow::Context,
	clap::Parser,
	ndsz_fat::{FileAllocationTable, FileNameTable},
	ndsz_util::{IoSlice, ReadByteArray},
	std::fs,
	tracing_subscriber::prelude::*,
};


fn main() -> Result<(), anyhow::Error> {
	// Initialize the logger
	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer().with_filter(tracing_subscriber::EnvFilter::from_default_env()))
		.init();

	// Get the arguments
	let args = Args::parse();

	// Get the output path
	let output_path = match args.output_path {
		Some(path) => path,
		None => args.input_path.with_extension(""),
	};

	// Open the rom
	let mut input_file = fs::File::open(&args.input_path).context("Unable to open input file")?;

	// Read the header
	let header = {
		let header_bytes = input_file.read_byte_array().context("Unable to read header")?;
		ndsz_nds::Header::from_bytes(&header_bytes).context("Unable to parse header")?
	};
	tracing::trace!(?header);

	// Get the fat
	let fat = {
		// Get the fat
		let mut fat_slice = IoSlice::new_with_offset_len(
			&mut input_file,
			u64::from(header.file_allocation_table.offset),
			u64::from(header.file_allocation_table.length),
		)
		.context("Unable to create fat slice")?;

		FileAllocationTable::from_reader(&mut fat_slice).context("Unable to get fat")?
	};

	// Output the fat, if requested
	if args.fat {
		let fat_path = output_path.join("fat.yaml");
		self::output_fat_yaml(&fat, fat_path).context("Unable to output fat yaml")?;
	}

	// Output the fat files, if requested
	if args.fat_files {
		self::extract_fat_raw(&fat, &mut input_file, &output_path)?;
	}

	// Get the fnt
	let fnt = {
		// Get the fnt
		let mut fnt_slice = IoSlice::new_with_offset_len(
			&mut input_file,
			u64::from(header.file_name_table.offset),
			u64::from(header.file_name_table.length),
		)
		.context("Unable to create fnt slice")?;

		// And read it
		FileNameTable::from_reader(&mut fnt_slice).context("Unable to read fnt")?
	};

	// Output the fnt, if requested
	if args.fnt {
		let fnt_path = output_path.join("fnt.yaml");
		self::output_fnt_yaml(&fnt, fnt_path).context("Unable to output fnt yaml")?;
	}

	// Create the output directory if it doesn't exist
	fs::create_dir_all(&output_path).context("Unable to create output directory")?;

	// Extract all nds files
	self::extract_all_parts(&mut input_file, &header, &output_path).context("Unable to extract parts")?;

	// Extract the filesystem
	let fs_dir = output_path.join("fs");
	self::extract_fat_dir(&fnt.root, &mut input_file, &fat, fs_dir).context("Unable to extract fat")?;

	Ok(())
}
