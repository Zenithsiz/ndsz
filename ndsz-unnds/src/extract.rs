//! Extraction

// Imports
use {
	anyhow::Context,
	ndsz_fat::{dir, Dir, FileAllocationTable},
	ndsz_util::{AsciiStrArr, IoSlice},
	std::{
		fs,
		io,
		path::{Path, PathBuf},
	},
};

/// Directory visitor
struct DirVisitor<'fat, 'reader, R> {
	/// Current path
	cur_path: PathBuf,

	/// Reader
	// Note: Has to be immutable due to a GAT bug.
	//       This also implies we can't use a BufReader.
	// TODO: Make this `&'reader mut R` once the GAT bug is solved.
	reader: &'reader mut R,

	/// The fat
	fat: &'fat FileAllocationTable,
}

impl<'fat, 'reader, R: io::Read + io::Seek> dir::Visitor for DirVisitor<'fat, 'reader, R> {
	type Error = anyhow::Error;
	type SubDirVisitor<'visitor, 'entry> = DirVisitor<'fat, 'visitor, R>
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
pub fn extract_fat_dir<R: io::Read + io::Seek>(
	dir: &Dir,
	reader: &mut R,
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

/// Extracts the fat without the fnt
pub fn extract_fat_raw<R: io::Read + io::Seek>(
	fat: &FileAllocationTable,
	rom_file: &mut R,
	output_path: &Path,
) -> Result<(), anyhow::Error> {
	let fat_dir = output_path.join("fat");
	fs::create_dir_all(&fat_dir).context("Unable to create fat output directory")?;
	for (idx, ptr) in fat.ptrs.iter().enumerate() {
		let name = format!("{idx}.bin");
		self::extract_part(
			rom_file,
			ptr.start_address,
			ptr.end_address - ptr.start_address,
			&name,
			&fat_dir,
		)
		.with_context(|| format!("Unable to extract fat entry #{idx} ({ptr:?})"))?;
	}

	Ok(())
}

/// Extract all parts of the nds header, except the filesystem
pub fn extract_all_parts<R: io::Read + io::Seek>(
	rom_file: &mut R,
	header: &ndsz_nds::Header,
	path: &Path,
) -> Result<(), anyhow::Error> {
	let parts = [
		(0x15, 0x7, "reserved1"),
		(0x88, 0x38, "reserved2"),
		(0x160, 0x20, "reserved_debugger"),
		(0xc0, 0x9c, "nintendo_logo"),
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
fn extract_part<R: io::Read + io::Seek>(
	rom_file: &mut R,
	offset: u32,
	size: u32,
	name: &str,
	path: &Path,
) -> Result<(), anyhow::Error> {
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
