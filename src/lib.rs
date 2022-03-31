use serde::{Deserialize, Serialize};

pub mod bxcad;
pub(crate) mod bytestream_addon;

pub use bxcad::{bccad::BCCAD, brcad::BRCAD, BXCAD};

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

pub struct VarLenString(String);
