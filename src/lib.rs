//! A crate with data types for known BXCAD (BRCAD/BCCAD) formats
//!
//! # Obtaining and working with BXCAD formats
//! In order to parse a BXCAD file's data, you must use the methods implemented in the [BXCAD] trait.
//!
//! **Example:**
//! ```no_run
//! use std::fs::File;
//! use flour::{BCCAD, BXCAD};
//! # fn main() -> Result<(), flour::error::Error> {
//!
//! // Open a BCCAD file
//! let mut file = File::open("file.bccad")?;
//! let mut bccad = BCCAD::from_binary(&mut file)?;
//!
//! // Delete all animations, and save as a new file
//! bccad.animations = vec![];
//! let mut out_file = File::create("file.out.bccad")?;
//! bccad.to_binary(&mut out_file)?;
//! # Ok(())
//! # }
//! ```
//!
//! # JSON (de)serializing and BXCADWrapper
//!
//! # Features
//! * **`modder_qol`**

use serde::{Deserialize, Serialize};

/// Contains a model for the generic BXCAD format, as well as
/// known implementations of it
pub mod bxcad;
/// Error handling
pub mod error;

pub(crate) mod bytestream_addon;

pub use bxcad::{bccad::BCCAD, brcad::BRCAD, BXCAD};

/// RGB color
#[derive(Serialize, Deserialize, Clone, Debug)]
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
