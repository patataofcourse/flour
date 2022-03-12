use crate::{Color, VarLenString};
use bytestream::*;
use std::{
    io::{Error, ErrorKind, Read, Result, Write},
    marker::Sized,
};

pub trait ByteStream {
    fn read_from(file: &mut dyn Read, order: ByteOrder) -> Result<Self>
    where
        Self: Sized;
    fn write_to(&self, file: &mut dyn Write, order: ByteOrder) -> Result<()>
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
    fn write_to(&self, file: &mut dyn Write, order: ByteOrder) -> Result<()> {
        let bytes = match order {
            ByteOrder::BigEndian => self.to_be_bytes(),
            ByteOrder::LittleEndian => self.to_le_bytes(),
        };
        file.write(&bytes)?;
        Ok(())
    }
}

impl StreamReader for Color {
    fn read_from<T: Read>(file: &mut T, order: ByteOrder) -> Result<Self> {
        let red = u8::read_from(file, order)?;
        let green = u8::read_from(file, order)?;
        let blue = u8::read_from(file, order)?;
        Ok(Self { red, green, blue })
    }
}

impl StreamWriter for Color {
    fn write_to<W: Write>(&self, file: &mut W, order: ByteOrder) -> Result<()> {
        self.red.write_to(file, order)?;
        self.green.write_to(file, order)?;
        self.blue.write_to(file, order)?;
        Ok(())
    }
}

impl StreamReader for VarLenString {
    fn read_from<T: Read>(file: &mut T, order: ByteOrder) -> Result<Self> {
        let mut bytes = vec![];
        let size = u8::read_from(file, order)?;
        for _ in 0..size {
            bytes.push(u8::read_from(file, order)?);
        }
        let padding_size = 4 - ((size + 1) % 4);
        for _ in 0..padding_size {
            u8::read_from(file, order)?;
        }
        let string = match String::from_utf8(bytes) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Error reading string");
                Err(Error::from(ErrorKind::Other))?
            }
        };
        Ok(Self(string))
    }
}
