//! Common library for `.nds` interaction

// Modules
pub mod header;
pub mod unit_code;

// Exports
pub use self::{header::Header, unit_code::UnitCode};
