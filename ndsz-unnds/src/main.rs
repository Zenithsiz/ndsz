//! Unpacks a `.nds`

// Features
#![feature(fs_try_exists, never_type, unwrap_infallible)]

// Modules
mod args;
mod extract;

// Imports
use {
	self::{
		args::Args,
		extract::{extract_all_parts, extract_fat_dir, extract_fat_hidden},
		yaml::output_yaml,
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
	let header = self::parse_header(&mut input_file)?;
	tracing::trace!(?header);

	// Parses the fat and fnt
	let fat = self::parse_fat(&mut input_file, &header)?;
	let fnt = self::parse_fnt(&mut input_file, &header)?;

	// Then extract all parts, as well as files not mentioned in the fnt
	fs::create_dir_all(&output_path).context("Unable to create output directory")?;
	self::extract_all_parts(&mut input_file, &header, &output_path).context("Unable to extract parts")?;
	let hidden_fat_files = self::extract_fat_hidden(&fat, &fnt, &mut input_file, &output_path)?;

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
