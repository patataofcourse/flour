use bytestream::{ByteOrder, StreamReader};
use serde::{Deserialize, Serialize};
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
