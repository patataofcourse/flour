use serde_derive::{Deserialize, Serialize};

pub mod bccad;
pub(crate) mod bytestream_addon;

pub use bccad::BCCAD;

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}
