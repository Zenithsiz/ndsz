//! Header

// Modules
mod error;

// Exports
pub use error::FromBytesError;

// Imports
use {
	crate::UnitCode,
	byteorder::{ByteOrder, LittleEndian},
	zutil::{ascii_str_arr::AsciiChar, AsciiStrArr},
};

/// Header
// From `http://dsibrew.org/wiki/DSi_cartridge_header`.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Header {
	/// Game title
	pub game_title: AsciiStrArr<0xc>,

	/// Game code
	pub game_code: AsciiStrArr<0x4>,

	/// Maker code
	pub maker_code: AsciiStrArr<0x2>,

	/// Unit code
	pub unit_code: UnitCode,

	/// Encryption seed select
	pub encryption_seed_select: u8,

	/// Device capacity
	pub device_capacity: u8,

	/// Reserved 1
	pub reserved1: [u8; 7],

	/// Game revision
	pub game_revision: u16,

	/// ROM version
	pub rom_version: u8,

	/// Internal flags
	pub internal_flags: u8,

	/// Arm 9 load data
	pub arm9_load_data: ArmLoadData,

	/// Arm 7 load data
	pub arm7_load_data: ArmLoadData,

	/// File name table
	pub file_name_table: TableLoadData,

	/// File allocation table
	pub file_allocation_table: TableLoadData,

	/// arm 9 overlay table
	pub arm9_overlay_table: TableLoadData,

	/// arm 7 overlay table
	pub arm7_overlay_table: TableLoadData,

	/// Normal card control registers settings
	pub normal_card_control_register_settings: u32,

	/// Secure card control registers settings
	pub secure_card_control_register_settings: u32,

	/// Icon banner offset
	pub icon_banner_offset: u32,

	/// Secure area CRC
	pub secure_area_crc: u16,

	/// Secure transfer timeout
	pub secure_transfer_timeout: u16,

	/// Arm9 auto-load
	pub arm9_auto_load: u32,

	/// Arm7 auto-load
	pub arm7_auto_load: u32,

	/// Secure disable
	pub secure_disable: u64,

	/// NTR region rom size
	pub ntr_region_rom_size: u32,

	/// Header size
	pub header_size: u32,

	/// Reserved 2
	pub reserved2: [u8; 0x38],

	/// Nintendo logo
	pub nintendo_logo: [u8; 0x9c],

	/// Nintendo logo crc
	pub nintendo_logo_crc: u16,

	/// Header crc
	pub header_crc: u16,

	/// Reserved for debugger
	pub reserved_debugger: [u8; 0x20],
}

impl Header {
	/// Parses a header data from bytes
	pub fn from_bytes(bytes: &[u8; 0x180]) -> Result<Self, FromBytesError> {
		let bytes = ndsz_bytes::array_split!(bytes,
			game_title                            : [0xc],  // 0x0
			game_code                             : [0x4],  // 0xc
			maker_code                            : [0x2],  // 0x10
			unit_code                             :  0x1 ,  // 0x12
			encryption_seed_select                :  0x1 ,  // 0x13
			device_capacity                       :  0x1 ,  // 0x14
			reserved1                             : [0x7],  // 0x15
			game_revision                         : [0x2],  // 0x1c
			rom_version                           :  0x1 ,  // 0x1e
			internal_flags                        :  0x1 ,  // 0x1f
			arm9_load_data                        : [0x10], // 0x20
			arm7_load_data                        : [0x10], // 0x30
			file_name_table                       : [0x8],  // 0x40
			file_allocation_table                 : [0x8],  // 0x48
			arm9_overlay_table                    : [0x8],  // 0x50
			arm7_overlay_table                    : [0x8],  // 0x58
			normal_card_control_register_settings : [0x4],  // 0x60
			secure_card_control_register_settings : [0x4],  // 0x64
			icon_banner_offset                    : [0x4],  // 0x68
			secure_area_crc                       : [0x2],  // 0x6c
			secure_transfer_timeout               : [0x2],  // 0x6e
			arm9_auto_load                        : [0x4],  // 0x70
			arm7_auto_load                        : [0x4],  // 0x74
			secure_disable                        : [0x8],  // 0x7c
			ntr_region_rom_size                   : [0x4],  // 0x80
			header_size                           : [0x4],  // 0x84
			reserved2                             : [0x38], // 0x88
			nintendo_logo                         : [0x9c], // 0xc0
			nintendo_logo_crc                     : [0x2],  // 0x15c
			header_crc                            : [0x2],  // 0x15e
			reserved_debugger                     : [0x20], // 0x160
		);

		Ok(Self {
			game_title: AsciiStrArr::from_bytes(bytes.game_title)
				.map_err(FromBytesError::GameTitle)?
				.trimmed_end(AsciiChar::Null),
			game_code: AsciiStrArr::from_bytes(bytes.game_code).map_err(FromBytesError::GameCode)?,
			maker_code: AsciiStrArr::from_bytes(bytes.maker_code).map_err(FromBytesError::MakerCode)?,
			unit_code: UnitCode::from_bytes(bytes.unit_code).ok_or(FromBytesError::UnitCode)?,
			encryption_seed_select: *bytes.encryption_seed_select,
			device_capacity: *bytes.device_capacity,
			reserved1: *bytes.reserved1,
			game_revision: LittleEndian::read_u16(bytes.game_revision),
			rom_version: *bytes.rom_version,
			internal_flags: *bytes.internal_flags,
			arm9_load_data: ArmLoadData::from_bytes(bytes.arm9_load_data),
			arm7_load_data: ArmLoadData::from_bytes(bytes.arm7_load_data),
			file_name_table: TableLoadData::from_bytes(bytes.file_name_table),
			file_allocation_table: TableLoadData::from_bytes(bytes.file_allocation_table),
			arm9_overlay_table: TableLoadData::from_bytes(bytes.arm9_overlay_table),
			arm7_overlay_table: TableLoadData::from_bytes(bytes.arm9_overlay_table),
			normal_card_control_register_settings: LittleEndian::read_u32(bytes.normal_card_control_register_settings),
			secure_card_control_register_settings: LittleEndian::read_u32(bytes.secure_card_control_register_settings),
			icon_banner_offset: LittleEndian::read_u32(bytes.icon_banner_offset),
			secure_area_crc: LittleEndian::read_u16(bytes.secure_area_crc),
			secure_transfer_timeout: LittleEndian::read_u16(bytes.secure_transfer_timeout),
			arm9_auto_load: LittleEndian::read_u32(bytes.arm9_auto_load),
			arm7_auto_load: LittleEndian::read_u32(bytes.arm7_auto_load),
			secure_disable: LittleEndian::read_u64(bytes.secure_disable),
			ntr_region_rom_size: LittleEndian::read_u32(bytes.ntr_region_rom_size),
			header_size: LittleEndian::read_u32(bytes.header_size),
			reserved2: *bytes.reserved2,
			nintendo_logo: *bytes.nintendo_logo,
			nintendo_logo_crc: LittleEndian::read_u16(bytes.nintendo_logo_crc),
			header_crc: LittleEndian::read_u16(bytes.header_crc),
			reserved_debugger: *bytes.reserved_debugger,
		})
	}
}

/// Table load data
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct TableLoadData {
	/// Offset
	pub offset: u32,

	/// Length
	pub length: u32,
}

impl TableLoadData {
	/// Parses a table load data from bytes
	pub fn from_bytes(bytes: &[u8; 8]) -> Self {
		let bytes = ndsz_bytes::array_split!(bytes,
			offset: [0x4],
			length: [0x4],
		);

		Self {
			offset: LittleEndian::read_u32(bytes.offset),
			length: LittleEndian::read_u32(bytes.length),
		}
	}
}

/// Arm load data
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct ArmLoadData {
	/// Offset
	pub offset: u32,

	/// Entry address
	pub entry_address: u32,

	/// Load address
	pub load_address: u32,

	/// Size
	pub size: u32,
}

impl ArmLoadData {
	/// Parses load data from bytes
	pub fn from_bytes(bytes: &[u8; 16]) -> Self {
		let bytes = ndsz_bytes::array_split!(bytes,
			offset       : [0x4],
			entry_address: [0x4],
			load_address : [0x4],
			size         : [0x4],
		);

		Self {
			offset:        LittleEndian::read_u32(bytes.offset),
			entry_address: LittleEndian::read_u32(bytes.entry_address),
			load_address:  LittleEndian::read_u32(bytes.load_address),
			size:          LittleEndian::read_u32(bytes.size),
		}
	}
}
