//! Utilities
//!
//! Most here might be moved to external creates if deemed worth it

// Features
#![feature(slice_index_methods)]

// Modules
pub mod ascii_str_arr;

// Exports
pub use ascii_str_arr::AsciiStrArr;

/// Helper to read an array of bytes from a reader
pub trait ReadByteArray {
	/// Reads a byte array, `[u8; N]` from this reader
	fn read_byte_array<const N: usize>(&mut self) -> Result<[u8; N], std::io::Error>;
}

impl<R: ?Sized + std::io::Read> ReadByteArray for R {
	fn read_byte_array<const N: usize>(&mut self) -> Result<[u8; N], std::io::Error> {
		let mut bytes = [0; N];
		self.read_exact(&mut bytes)?;
		Ok(bytes)
	}
}
