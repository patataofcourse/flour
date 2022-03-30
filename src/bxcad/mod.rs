use bytestream::{ByteOrder, StreamReader};
use serde::{Deserialize, Serialize};
use serde_derive::{Deserialize, Serialize};
use std::io::{Read, Result as IOResult, Seek, SeekFrom, Write};

pub mod bccad;
pub mod brcad;

pub trait BXCAD<'a>: Serialize + Deserialize<'a> {
    const BYTE_ORDER: ByteOrder;
    const TIMESTAMP: u32;
    fn from_binary<F: Read + Seek>(f: &mut F) -> IOResult<Self>;
    fn to_binary<F: Write>(&self, f: &mut F) -> IOResult<()>;
    fn is_format<F: Read + Seek>(f: &mut F) -> IOResult<bool> {
        let timestamp = u32::read_from(f, Self::BYTE_ORDER)?;
        f.seek(SeekFrom::Current(-4))?;
        Ok(timestamp == Self::TIMESTAMP)
    }
}

pub enum BXCADType {
    BRCAD,
    BCCAD,
    None,
}

pub fn get_bxcad_type<'a, F: Read + Seek>(f: &mut F) -> IOResult<BXCADType> {
    if bccad::BCCAD::is_format(f)? {
        Ok(BXCADType::BCCAD)
    // } else if brcad::BRCAD::is_format(f)? {
    //     Ok(BXCADType::BRCAD)
    } else {
        Ok(BXCADType::None)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PosInTexture {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
