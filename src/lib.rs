use serde::{Deserialize, Serialize};

/// Contains a model for the generic BXCAD format, as well as
/// known implementations of it
pub mod bxcad;
/// Error handling
pub mod error;

pub(crate) mod bytestream_addon;

pub use bxcad::{bccad::BCCAD, brcad::BRCAD, BXCAD};

/// Color in RGB form
#[derive(Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

/// Variable length string, used in BCCAD labels
/// 
/// Format is as follows:
///
/// * 1 byte for string size (n)
/// * n bytes for string contents
/// * Padded to 4 bytes
pub struct VarLenString(String);
