//! A crate with data types for known BXCAD (BRCAD/BCCAD) formats
//!
//! # Obtaining and working with BXCAD formats
//! In order to parse a BXCAD file's data, you must use the methods implemented in the [BXCAD] trait.
//!
//! **Example:**
//! ```no_run
//! # use std::fs::File;
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
//! # Serializing and BXCADWrapper
//! While [`BXCAD`] doesn't require `serde::Serialize` and `serde::Deserialize`, implementing
//! those is the main idea behind the trait, since that way the BXCAD can be wrapped inside a
//! [`bxcad::BXCADWrapper`]. The function of this struct is to give a common metadata API for all
//! flour-compatible BXCADs.
//!
//! You can create a [`BXCADWrapper`][bxcad::BXCADWrapper] like so:
//! ```no_run
//! # use std::fs::File;
//! # use flour::{BCCAD, BXCAD};
//! use flour::bxcad::BXCADWrapper;
//! # fn main() -> Result<(), flour::error::Error> {
//! # let mut file = File::open("file.bccad")?;
//! # let mut bccad = BCCAD::from_binary(&mut file)?;
//!
//! // this could instead be BXCADWrapper::from_bxcad_indexized, to allow indexization
//! let wrapper = BXCADWrapper::from_bxcad(bccad);
//! 
//! # Ok(())
//! # }
//! ```
//!
//! This [`BXCADWrapper`][bxcad::BXCADWrapper] can now be serialized/deserialized without any
//! compatibility issues with other BXCAD types or with future/past versions of flour (post-1.0).
//!
//! For details on "indexization", see [`bxcad::qol::Indexizable`].
//!
//! # Comments
//! `flour` supports the following types of comments:
//! - Single-line: `// comment`
//! - Multi-line: `/* comment */`
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
