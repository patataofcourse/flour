use crate::error::{Error, Result};
use bytestream::{ByteOrder, StreamReader};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Seek, SeekFrom, Write};

/// Everything related to the BCCAD format used in Rhythm Heaven Megamix
pub mod bccad;
/// Everything related to the BRCAD format used in Rhythm Heaven Fever
pub mod brcad;

pub const OLDEST_SUPPORTED_VERSION: &'static str = "2.0.0-pre1";

/// A trait that encapsulates the basics of the B_CAD or BXCAD formats,
/// meant to be used for ease of (de)serializing
pub trait BXCAD<'a>: Serialize + for<'de> Deserialize<'de> {
    /// Endianness of the file
    const BYTE_ORDER: ByteOrder;
    /// Last revision timestamp - used to identify different versions
    /// of the format
    const TIMESTAMP: u32;
    /// Variant of the BXCADType enum that applies to it
    const BXCAD_TYPE: BXCADType;
    /// Function that takes a buffer and interprets its contents as the
    /// given BXCAD format
    fn from_binary<F: Read + Seek>(f: &mut F) -> Result<Self>;
    /// Function that creates the binary representation of the BXCAD file
    /// from its definition
    fn to_binary<F: Write>(&self, f: &mut F) -> Result<()>;
    /// Checks whether a given buffer contains BXCAD data for the
    /// given format
    fn is_format<F: Read + Seek>(f: &mut F) -> Result<bool> {
        let timestamp = u32::read_from(f, Self::BYTE_ORDER)?;
        f.seek(SeekFrom::Current(-4))?;
        Ok(timestamp == Self::TIMESTAMP)
    }
}

/// An enum of all the different types of BXCAD supported by this library
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[non_exhaustive]
pub enum BXCADType {
    /// BRCAD, used with Rhythm Heaven Fever. See [`brcad`]
    BRCAD,
    /// BCCAD, used with Rhythm Heaven Megamix. See [`bccad`]
    BCCAD,
    /// Any other BXCAD datatype supported by flour
    //TODO: add timestamp and byteorder fields
    Custom(
        /// Identifier for the BXCAD datatype
        String,
    ),
}

//TODO: get_bxcad_type_or_custom
/// Returns the BXCAD type associated with the given file, or an error if none
pub fn get_bxcad_type<'a, F: Read + Seek>(f: &mut F) -> Result<BXCADType> {
    if bccad::BCCAD::is_format(f)? {
        Ok(BXCADType::BCCAD)
    } else if brcad::BRCAD::is_format(f)? {
        Ok(BXCADType::BRCAD)
    } else {
        Err(Error::NotBXCAD)
    }
}

/// Bounding box for a sprite part's texture in the texture sheet
#[derive(Serialize, Deserialize)]
pub struct PosInTexture {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

/// A wrapper that contains data about the BXCAD file, meant to be used
/// with serializing/deserializing (see the flour main executable)
#[derive(Serialize, Deserialize)]
pub struct BXCADWrapper {
    /// Type of the BXCAD file
    pub bxcad_type: BXCADType,
    /// flour version that was used to create this file (SemVer)
    pub flour_version: String,
    /// Actual BXCAD data as a serde value
    pub data: Value,
}

impl BXCADWrapper {
    /// Create a BXCADWrapper from the given BXCAD struct
    pub fn from_bxcad<'a, T: BXCAD<'a>>(bxcad: T) -> Self {
        Self {
            bxcad_type: T::BXCAD_TYPE,
            flour_version: env!("CARGO_PKG_VERSION").to_string(),
            data: serde_json::to_value(bxcad).unwrap(),
        }
    }
    /// Return the wrapper's BXCAD data if compatible
    pub fn to_bxcad<'a, T: BXCAD<'a>>(self) -> Result<T> {
        let requirement = VersionReq::parse(&format!(
            "<={}, >={}",
            env!("CARGO_PKG_VERSION"),
            OLDEST_SUPPORTED_VERSION
        ))?;
        let version = Version::parse(&self.flour_version)?; //TODO: add specific error

        if !requirement.matches(&version) {
            //TODO: add specific error
            return Err(Error::IncompatibleVersion(version));
        }

        //TODO: this might false-positive some bxcads
        Ok(serde_json::from_value(self.data)?)
    }
}
