use bytestream::ByteOrder;
use std::{
    io::{Read, Result},
    marker::Sized,
};

pub trait ByteStream {
    fn read_from(file: &mut dyn Read, order: ByteOrder) -> Result<Self>
    where
        Self: Sized;
}

impl ByteStream for f32 {
    fn read_from(file: &mut dyn Read, order: ByteOrder) -> Result<Self> {
        let mut bytes = [0; 4];
        file.read_exact(&mut bytes)?;
        match order {
            ByteOrder::BigEndian => Ok(Self::from_be_bytes(bytes)),
            ByteOrder::LittleEndian => Ok(Self::from_le_bytes(bytes)),
        }
    }
}
