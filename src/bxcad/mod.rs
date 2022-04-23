use crate::error::Result;
use bytestream::{ByteOrder, StreamReader};
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{Read, Seek, SeekFrom, Write};

pub mod bccad;
pub mod brcad;

pub trait BXCAD<'a>: Serialize + for<'de> Deserialize<'de> {
    const BYTE_ORDER: ByteOrder;
    const TIMESTAMP: u32;
    const BXCAD_TYPE: BXCADType;
    fn from_binary<F: Read + Seek>(f: &mut F) -> Result<Self>;
    fn to_binary<F: Write>(&self, f: &mut F) -> Result<()>;
    fn is_format<F: Read + Seek>(f: &mut F) -> Result<bool> {
        let timestamp = u32::read_from(f, Self::BYTE_ORDER)?;
        f.seek(SeekFrom::Current(-4))?;
        Ok(timestamp == Self::TIMESTAMP)
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum BXCADType {
    BRCAD,
    BCCAD,
    Custom(String),
}

pub fn get_bxcad_type<'a, F: Read + Seek>(f: &mut F) -> Result<BXCADType> {
    if bccad::BCCAD::is_format(f)? {
        Ok(BXCADType::BCCAD)
    } else if brcad::BRCAD::is_format(f)? {
        Ok(BXCADType::BRCAD)
    } else {
        Ok(BXCADType::Custom("unsupported".to_string()))
    }
}

#[derive(Serialize, Deserialize)]
pub struct PosInTexture {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Serialize, Deserialize)]
pub struct BXCADWrapper {
    pub bxcad_type: BXCADType,
    pub flour_version: String,
    pub data: Value,
}

impl BXCADWrapper {
    pub fn from_bxcad<'a, T: BXCAD<'a>>(bxcad: T) -> Self {
        Self {
            bxcad_type: T::BXCAD_TYPE,
            flour_version: env!("CARGO_PKG_VERSION").to_string(),
            data: serde_json::to_value(bxcad).unwrap(),
        }
    }
    pub fn to_bxcad<'a, T: BXCAD<'a>>(self) -> Option<T> {
        let requirement =
            VersionReq::parse(&format!("<={}, >=2.0", env!("CARGO_PKG_VERSION"))).unwrap();
        let version = Version::parse(&self.flour_version).ok()?; //TODO: add specific error

        if !requirement.matches(&version) {
            //TODO: add specific error
            return None;
        }

        //TODO: this might false-positive some bxcads
        match serde_json::from_value(self.data) {
            Ok(c) => Some(c),
            Err(_) => None, //TODO: add specific error
        }
    }
}
