//! Unit code

/// Unit code
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum UnitCode {
	Nds,
	NdsDsi,
	Dsi,
}

impl UnitCode {
	/// Parses a unit code from bytes
	pub fn from_bytes(byte: &u8) -> Option<Self> {
		let code = match byte {
			0x0 => Self::Nds,
			0x2 => Self::NdsDsi,
			0x3 => Self::Dsi,
			_ => return None,
		};

		Some(code)
	}
}
