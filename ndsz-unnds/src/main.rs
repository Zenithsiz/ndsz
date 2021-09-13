//! Unpacks a `.nds`

// Features
#![feature(format_args_capture, generic_associated_types, path_try_exists)]

// Modules
mod args;

// Imports
use anyhow::Context;
use ndsz_fat::{dir, Dir, FileAllocationTable, FileNameTable};
use std::{
	fs, io,
	path::{Path, PathBuf},
};
use zutil::{AsciiStrArr, IoSlice, ReadByteArray};


fn main() -> Result<(), anyhow::Error> {
	// Initialize the logger
	simplelog::TermLogger::init(
		log::LevelFilter::Debug,
		simplelog::Config::default(),
		simplelog::TerminalMode::Stderr,
		simplelog::ColorChoice::Auto,
	)
	.context("Unable to initialize logger")?;

	// Get the arguments
	let args = args::get().context("Unable to retrieve arguments")?;

	// Open the rom
	let mut rom_file = fs::File::open(args.game_path).context("Unable to open game file")?;

	// Read the header
	let header = {
		let header_bytes = rom_file.read_byte_array().context("Unable to read rom header")?;
		ndsz_nds::Header::from_bytes(&header_bytes).context("Unable to parse rom header")?
	};

	log::info!("Found header {header:#?}");

	// Get the fat
	let fat = {
		// Get the fat
		let mut fat_slice = IoSlice::new_with_offset_len(
			&mut rom_file,
			u64::from(header.file_allocation_table.offset),
			u64::from(header.file_allocation_table.length),
		)
		.context("Unable to create fat slice")?;

		FileAllocationTable::from_reader(&mut fat_slice).context("Unable to get fat")?
	};

	// Get the fnt
	let fnt = {
		// Get the fnt
		let mut fnt_slice = IoSlice::new_with_offset_len(
			&mut rom_file,
			u64::from(header.file_name_table.offset),
			u64::from(header.file_name_table.length),
		)
		.context("Unable to create fnt slice")?;

		// And read it
		FileNameTable::from_reader(&mut fnt_slice).context("Unable to read fnt")?
	};

	// Create the output directory if it doesn't exist
	if !fs::try_exists(&args.output_path).context("Unable to check if output directory exists")? {
		fs::create_dir_all(&args.output_path).context("Unable to create directory")?;
	}

	// Extract all nds files
	self::extract_all_parts(&rom_file, &header, &args.output_path).context("Unable to extract parts")?;

	// Extract the filesystem
	let fs_dir = args.output_path.join("fs");
	self::extract_fat_dir(&fnt.root, &rom_file, &fat, fs_dir).context("Unable to extract fat")?;

	Ok(())
}

/// Directory visitor
struct DirVisitor<'fat, 'reader> {
	/// Current path
	cur_path: PathBuf,

	/// Reader
	// Note: Has to be immutable due to a GAT bug.
	//       This also implies we can't use a BufReader.
	// TODO: Make this `&'reader mut R` once the GAT bug is solved.
	reader: &'reader fs::File,

	/// The fat
	fat: &'fat FileAllocationTable,
}

impl<'fat, 'reader> dir::Visitor for DirVisitor<'fat, 'reader> {
	type Error = anyhow::Error;
	type SubDirVisitor<'visitor, 'entry> = DirVisitor<'fat, 'reader>;

	fn visit_file(&mut self, name: &AsciiStrArr<0x80>, id: u16) -> Result<(), Self::Error> {
		let path = self.cur_path.join(name.as_str());
		println!("{}", path.display());

		// Get the file on the rom
		let rom_file_ptr = &self.fat.ptrs[usize::from(id)];
		let mut rom_file = IoSlice::new(
			&mut self.reader,
			u64::from(rom_file_ptr.start_address)..u64::from(rom_file_ptr.end_address),
		)
		.context("Unable to read rom file")?;

		// Then copy it to disk
		let mut output_file = fs::File::create(&path).context("Unable to create output file")?;
		io::copy(&mut rom_file, &mut output_file).context("Unable to write to output file")?;

		Ok(())
	}

	fn visit_dir<'visitor, 'entry>(
		&'visitor mut self, name: &'entry AsciiStrArr<0x80>, _id: u16,
	) -> Result<Self::SubDirVisitor<'visitor, 'entry>, Self::Error> {
		let path = self.cur_path.join(name.as_str());
		println!("{}", path.display());

		// Create the directory
		fs::create_dir_all(&path).context("Unable to create directory")?;

		Ok(DirVisitor {
			cur_path: path,
			reader:   &mut self.reader,
			fat:      &*self.fat,
		})
	}
}

/// Extracts all files from a fat directory
fn extract_fat_dir(
	dir: &Dir, reader: &fs::File, fat: &FileAllocationTable, path: PathBuf,
) -> Result<(), anyhow::Error> {
	let mut visitor = DirVisitor {
		fat,
		reader,
		cur_path: path,
	};
	dir.walk(&mut visitor).context("Unable to extract root directory")
}


/// Extract all parts of the nds header, except the filesystem
fn extract_all_parts(rom_file: &fs::File, header: &ndsz_nds::Header, path: &Path) -> Result<(), anyhow::Error> {
	let var_name = [
		(
			header.arm9_load_data.offset,
			header.arm9_load_data.size,
			"arm9_load_data",
		),
		(
			header.arm7_load_data.offset,
			header.arm7_load_data.size,
			"arm7_load_data",
		),
		(
			header.arm9_overlay_table.offset,
			header.arm9_overlay_table.length,
			"arm9_overlay_table",
		),
		(
			header.arm7_overlay_table.offset,
			header.arm7_overlay_table.length,
			"arm7_overlay_table",
		),
	];

	for (offset, size, name) in var_name {
		self::extract_part(rom_file, offset, size, name, path).with_context(|| format!("Unable to extract {name}"))?;
	}

	Ok(())
}

/// Extracts a part given it's offset and size from the game file
fn extract_part(rom_file: &fs::File, offset: u32, size: u32, name: &str, path: &Path) -> Result<(), anyhow::Error> {
	let mut file_path = path.join(name);
	file_path.set_extension("bin");
	println!("{}", file_path.display());

	// Get the slice from the rom file
	let mut slice = IoSlice::new_with_offset_len(rom_file, u64::from(offset), u64::from(size))
		.with_context(|| format!("Unable to create {name} slice"))?;

	// Then create the output file
	let mut file = fs::File::create(file_path).with_context(|| format!("Unable to create {name} file"))?;

	// And copy them
	io::copy(&mut slice, &mut file).with_context(|| format!("Unable to write {name} to file"))?;

	Ok(())
}
