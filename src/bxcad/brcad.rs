use crate::{
    bxcad::{PosInTexture, BXCAD},
    bytestream_addon::ByteStream,
    Color, VarLenString,
};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use serde_derive::{Deserialize, Serialize};
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
    pub unk1: u8, //terminator?
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
    fn from_binary<F: Read + Seek>(f: &mut F) -> IOResult<Self> {
        unimplemented!();
    }
    fn to_binary<F: Write>(&self, f: &mut F) -> IOResult<()> {
        unimplemented!();
    }
}
