//! Unpacks a `.narc`

// Features
#![feature(fs_try_exists)]

// Modules
mod args;

// Imports
use {
	self::args::Args,
	anyhow::Context,
	clap::Parser,
	ndsz_fat::{dir, Dir, FileAllocationTable},
	ndsz_narc::Narc,
	ndsz_util::AsciiStrArr,
	std::{fs, io, path::PathBuf},
	tracing_subscriber::prelude::*,
	zutil::IoSlice,
};


fn main() -> Result<(), anyhow::Error> {
	// Initialize the logger
	tracing_subscriber::registry()
		.with(tracing_subscriber::fmt::layer().with_filter(tracing_subscriber::EnvFilter::from_default_env()))
		.init();

	// Get the arguments
	let args = Args::parse();

	// Open the rom
	let rom_file = fs::File::open(&args.input_path).context("Unable to open game file")?;

	// Read the narc
	let narc = match args.narcless {
		true => Narc::narcless_from_reader(rom_file).context("Unable to read narc")?,
		false => Narc::from_reader(rom_file).context("Unable to read narc")?,
	};

	// Get the output path
	let output_path = match args.output_path {
		Some(path) => path,
		None => args.input_path.with_extension(""),
	};

	// Create the output directory if it doesn't exist
	fs::create_dir_all(&output_path).context("Unable to create directory")?;

	// Extract the filesystem
	match args.extract_fat_on_empty_fnt && narc.fnt.root.entries.is_empty() {
		true => self::extract_fat_entries::<IoSlice<fs::File>>(&narc.fat, &narc.data.0, output_path)
			.context("Unable to extract entries of fat")?,
		false => self::extract_fat_dir::<IoSlice<fs::File>>(&narc.fnt.root, &narc.data.0, &narc.fat, output_path)
			.context("Unable to extract fat")?,
	}

	Ok(())
}

/// Directory visitor
struct DirVisitor<'fat, 'reader, R> {
	/// Current path
	cur_path: PathBuf,

	/// Reader
	// Note: Has to be immutable due to a GAT bug.
	//       This also implies we can't use a BufReader.
	// TODO: Make this `&'reader mut R` once the GAT bug is solved.
	reader: &'reader R,

	/// The fat
	fat: &'fat FileAllocationTable,
}

impl<'fat, 'reader, R> dir::Visitor for DirVisitor<'fat, 'reader, R>
where
	&'reader R: io::Read + io::Seek,
{
	type Error = anyhow::Error;
	type SubDirVisitor<'visitor, 'entry> = DirVisitor<'fat, 'reader, R>
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
fn extract_fat_dir<'a, R>(
	dir: &Dir,
	reader: &'a R,
	fat: &FileAllocationTable,
	path: PathBuf,
) -> Result<(), anyhow::Error>
where
	&'a R: io::Read + io::Seek,
{
	let mut visitor = DirVisitor {
		fat,
		reader,
		cur_path: path,
	};
	dir.walk(&mut visitor).context("Unable to extract root directory")
}

/// Extracts all entries from a fat
fn extract_fat_entries<'a, R>(fat: &FileAllocationTable, reader: &'a R, path: PathBuf) -> Result<(), anyhow::Error>
where
	&'a R: io::Read + io::Seek,
{
	for (idx, rom_file_ptr) in fat.ptrs.iter().enumerate() {
		let path = path.join(&format!("{idx}.bin"));
		println!("{}", path.display());

		// Get the file on the rom
		let mut rom_file = IoSlice::new(
			reader,
			u64::from(rom_file_ptr.start_address)..u64::from(rom_file_ptr.end_address),
		)
		.context("Unable to read rom file")?;

		let mut output_file = fs::File::create(&path).context("Unable to create output file")?;
		io::copy(&mut rom_file, &mut output_file).context("Unable to write to output file")?;
	}

	Ok(())
}
