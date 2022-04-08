use crate::{
    bxcad::{BXCADType, PosInTexture, BXCAD},
    bytestream_addon::ByteStream,
    Color, VarLenString,
};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use serde::{Deserialize, Serialize};
use std::io::{Read, Result as IOResult, Seek, Write};

#[derive(Serialize, Deserialize)]
pub struct BRCAD {
    pub timestamp: Option<u32>,
    pub unk0: u32,
    pub spritesheet_num: u16,
    pub spritesheet_control: u16,
    pub texture_width: u16,
    pub texture_height: u16,
    pub sprites: Vec<Sprite>,
    pub animations: Vec<Animation>,
    pub labels_file: Option<BRCADLabels>,
}

#[derive(Serialize, Deserialize)]
pub struct BRCADLabels(pub Vec<String>);

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub parts: Vec<SpritePart>,
}

#[derive(Serialize, Deserialize)]
pub struct SpritePart {
    pub texture_pos: PosInTexture,
    pub unk0: u32,
    pub pos_x: u16,
    pub pos_y: u16,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
    pub opacity: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Animation {
    pub name: Option<String>,
    pub steps: Vec<AnimationStep>,
}

#[derive(Serialize, Deserialize)]
pub struct AnimationStep {
    pub sprite: u16,
    pub duration: u16,
    pub unk0: u32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub opacity: u8,
    pub unk1: [u8; 3],
}

impl BXCAD<'_> for BRCAD {
    const BYTE_ORDER: ByteOrder = ByteOrder::BigEndian;
    const TIMESTAMP: u32 = 20100312;
    const BXCAD_TYPE: BXCADType = BXCADType::BRCAD;
    fn from_binary<F: Read + Seek>(f: &mut F) -> IOResult<Self> {
        let timestamp = u32::read_from(f, Self::BYTE_ORDER)?;
        let unk0 = u32::read_from(f, Self::BYTE_ORDER)?;
        let spritesheet_num = u16::read_from(f, Self::BYTE_ORDER)?;
        let spritesheet_control = u16::read_from(f, Self::BYTE_ORDER)?;
        let texture_width = u16::read_from(f, Self::BYTE_ORDER)?;
        let texture_height = u16::read_from(f, Self::BYTE_ORDER)?;

        let sprite_count = u32::read_from(f, Self::BYTE_ORDER)?;
        let mut sprites = vec![];
        for _ in 0..sprite_count {
            let parts_count = u32::read_from(f, Self::BYTE_ORDER)?;
            let mut parts = vec![];
            for _ in 0..parts_count {
                let texture_pos = PosInTexture {
                    x: u16::read_from(f, Self::BYTE_ORDER)?,
                    y: u16::read_from(f, Self::BYTE_ORDER)?,
                    width: u16::read_from(f, Self::BYTE_ORDER)?,
                    height: u16::read_from(f, Self::BYTE_ORDER)?,
                };
                let unk0 = u32::read_from(f, Self::BYTE_ORDER)?;
                let pos_x = u16::read_from(f, Self::BYTE_ORDER)?;
                let pos_y = u16::read_from(f, Self::BYTE_ORDER)?;
                let scale_x = f32::read_from(f, Self::BYTE_ORDER)?;
                let scale_y = f32::read_from(f, Self::BYTE_ORDER)?;
                let rotation = f32::read_from(f, Self::BYTE_ORDER)?;
                let flip_x = bool::read_from(f, Self::BYTE_ORDER)?;
                let flip_y = bool::read_from(f, Self::BYTE_ORDER)?;
                let opacity = u8::read_from(f, Self::BYTE_ORDER)?;
                u8::read_from(f, Self::BYTE_ORDER)?; // terminator/padding
                parts.push(SpritePart {
                    texture_pos,
                    unk0,
                    pos_x,
                    pos_y,
                    scale_x,
                    scale_y,
                    rotation,
                    flip_x,
                    flip_y,
                    opacity,
                })
            }
            sprites.push(Sprite { parts });
        }
        unimplemented!();
    }
    fn to_binary<F: Write>(&self, f: &mut F) -> IOResult<()> {
        unimplemented!();
    }
}

impl BRCAD {
    pub fn apply_labels<F: Read>(&self, labels: &mut F) -> IOResult<()> {
        unimplemented!()
    }
}
