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
	std::{fs, io},
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
	let header = parse_header(&mut input_file)?;
	tracing::trace!(?header);

	// Get the fat, then output it, and it's files, if requested
	let fat = self::parse_fat(&mut input_file, &header)?;

	if args.fat {
		let fat_path = output_path.join("fat.yaml");
		self::output_fat_yaml(&fat, fat_path).context("Unable to output fat yaml")?;
	}

	if args.fat_files {
		self::extract_fat_raw(&fat, &mut input_file, &output_path)?;
	}

	// Get the fnt, then output it, if requested
	let fnt = self::parse_fnt(&mut input_file, &header)?;

	if args.fnt {
		let fnt_path = output_path.join("fnt.yaml");
		self::output_fnt_yaml(&fnt, fnt_path).context("Unable to output fnt yaml")?;
	}

	// Then extract all parts, and the directory structure
	fs::create_dir_all(&output_path).context("Unable to create output directory")?;

	self::extract_all_parts(&mut input_file, &header, &output_path).context("Unable to extract parts")?;

	let fs_dir = output_path.join("fs");
	self::extract_fat_dir(&fnt.root, &mut input_file, &fat, fs_dir).context("Unable to extract fat")?;

	Ok(())
}

/// Parses the header
fn parse_header<R: io::Read>(rom_file: &mut R) -> Result<ndsz_nds::Header, anyhow::Error> {
	let header_bytes = rom_file.read_byte_array().context("Unable to read header")?;
	ndsz_nds::Header::from_bytes(&header_bytes).context("Unable to parse header")
}

/// Parses the fat
fn parse_fat<R: io::Read + io::Seek>(
	rom_file: &mut R,
	header: &ndsz_nds::Header,
) -> Result<FileAllocationTable, anyhow::Error> {
	// Create the slice
	let mut fat_slice = IoSlice::new_with_offset_len(
		rom_file,
		u64::from(header.file_allocation_table.offset),
		u64::from(header.file_allocation_table.length),
	)
	.context("Unable to create fat slice")?;

	// Then parse it
	FileAllocationTable::from_reader(&mut fat_slice).context("Unable to get fat")
}

/// Parses the fnt
fn parse_fnt<R: io::Read + io::Seek>(
	rom_file: &mut R,
	header: &ndsz_nds::Header,
) -> Result<FileNameTable, anyhow::Error> {
	// Create the slice
	let mut fnt_slice = IoSlice::new_with_offset_len(
		rom_file,
		u64::from(header.file_name_table.offset),
		u64::from(header.file_name_table.length),
	)
	.context("Unable to create fnt slice")?;

	// Then parse it
	FileNameTable::from_reader(&mut fnt_slice).context("Unable to read fnt")
}
