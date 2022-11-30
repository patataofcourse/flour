use crate::error::{Error, Result};
use bytestream::{ByteOrder, StreamReader};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, SeekFrom, Write};

/// Everything related to the BCCAD format used in Rhythm Heaven Megamix
pub mod bccad;
/// Everything related to the BRCAD format used in Rhythm Heaven Fever
pub mod brcad;

/// QoL features for the JSON format
#[cfg(feature = "modder_qol")]
pub mod qol;

/// Oldest supported BXCAD version by the current ver.
pub const OLDEST_SUPPORTED_VERSION: &'static str = "2.0.0-pre1";

/// A trait that encapsulates the basics of the BCAD / BXCAD formats,
/// meant to be used for ease of (de)serializing
pub trait BXCAD<'de>: Serialize + Deserialize<'de> {
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
    //  /// Any other BXCAD datatype supported by flour
    //  //TODO: add timestamp and byteorder fields
    //  Custom(
    //      /// Identifier for the BXCAD datatype
    //      String,
    //  ),
}

//TODO: get_bxcad_type_or_custom
/// Returns the builtin BXCAD type associated with the given file, if any
pub fn get_bxcad_type<'a, F: Read + Seek>(f: &mut F) -> Result<Option<BXCADType>> {
    Ok(if bccad::BCCAD::is_format(f)? {
        Some(BXCADType::BCCAD)
    } else if brcad::BRCAD::is_format(f)? {
        Some(BXCADType::BRCAD)
    } else {
        None
    })
}

/// Bounding box for a sprite part's texture in the texture sheet
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PosInTexture {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

/// A wrapper that contains data about the BXCAD file, meant to be used
/// with serializing/deserializing (see the flour main executable)
#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct BXCADWrapper<X> {
    /// Type of the BXCAD file
    pub bxcad_type: BXCADType,
    /// flour version that was used to create this file (SemVer)
    pub flour_version: String,
    /// QoL options existing in the JSON file
    #[serde(default)]
    pub indexize: bool,
    /// Actual BXCAD data
    data: X,
}

impl<X: for<'de> BXCAD<'de>> BXCADWrapper<X> {
    /// Create a BXCADWrapper from the given BXCAD struct.
    ///
    /// To see the meaning of `indexize`, see [BXCADWrapper::indexize]
    pub fn from_bxcad(bxcad: X) -> Self {
        Self {
            bxcad_type: X::BXCAD_TYPE,
            flour_version: env!("CARGO_PKG_VERSION").to_string(),
            indexize: false,
            data: bxcad,
        }
    }

    /// Return the wrapper's BXCAD data if compatible
    pub fn to_bxcad(self) -> Result<X> {
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

        if self.indexize == true {
            if cfg!(not(feature = "modder_qol")) {
                todo!("ERROR HERE")
            } else {
                //Ok(T::from_indexized(self.data)?)
                todo!();
            }
        } else {
            //TODO: this might false-positive some bxcads
            Ok(self.data)
        }
    }
}

#[cfg(feature = "modder_qol")]
impl<X: qol::Indexizable> BXCADWrapper<X> {
    pub fn from_bxcad_indexize(bxcad: X) -> BXCADWrapper<X::Indexized> {
        let data = bxcad.to_indexized();
        BXCADWrapper {
            bxcad_type: X::BXCAD_TYPE,
            flour_version: env!("CARGO_PKG_VERSION").to_string(),
            indexize: true,
            data,
        }
    }
}
