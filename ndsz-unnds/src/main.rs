//! Unpacks a `.nds`

// Features
#![feature(fs_try_exists)]

// Modules
mod args;
mod debug_fat;
mod debug_fnt;

// Imports
use {
	self::{args::Args, debug_fat::output_fat_yaml, debug_fnt::output_fnt_yaml},
	anyhow::Context,
	clap::Parser,
	ndsz_fat::{dir, Dir, FileAllocationTable, FileNameTable},
	ndsz_util::{AsciiStrArr, IoSlice, ReadByteArray},
	std::{
		fs,
		io,
		path::{Path, PathBuf},
	},
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
		let fat_dir = output_path.join("fat");
		fs::create_dir_all(&fat_dir).context("Unable to create fat output directory")?;

		for (idx, ptr) in fat.ptrs.iter().enumerate() {
			let name = format!("{idx}.bin");
			self::extract_part(
				&input_file,
				ptr.start_address,
				ptr.end_address - ptr.start_address,
				&name,
				&fat_dir,
			)
			.with_context(|| format!("Unable to extract fat entry #{idx} ({ptr:?})"))?;
		}
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
	self::extract_all_parts(&input_file, &header, &output_path).context("Unable to extract parts")?;

	// Extract the filesystem
	let fs_dir = output_path.join("fs");
	self::extract_fat_dir(&fnt.root, &input_file, &fat, fs_dir).context("Unable to extract fat")?;

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
	type SubDirVisitor<'visitor, 'entry> = DirVisitor<'fat, 'reader>
	where
		Self: 'visitor;

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
		&'visitor mut self,
		name: &'entry AsciiStrArr<0x80>,
		_id: u16,
	) -> Result<Self::SubDirVisitor<'visitor, 'entry>, Self::Error> {
		let path = self.cur_path.join(name.as_str());
		println!("{}", path.display());

		// Create the directory
		fs::create_dir_all(&path).context("Unable to create directory")?;

		Ok(DirVisitor {
			cur_path: path,
			reader:   self.reader,
			fat:      self.fat,
		})
	}
}

/// Extracts all files from a fat directory
fn extract_fat_dir(
	dir: &Dir,
	reader: &fs::File,
	fat: &FileAllocationTable,
	path: PathBuf,
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
	let parts = [
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

	for (offset, size, name) in parts {
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
