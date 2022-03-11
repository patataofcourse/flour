use crate::bytestream_addon::ByteStream;
use bytestream::{ByteOrder, StreamReader};
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::Result as IOResult};

#[derive(Serialize, Deserialize)]
pub struct BCCAD {
    pub timestamp: u32,
    pub texture_width: u16,
    pub texture_height: u16,
    pub sprites: Vec<Sprite>,
    pub animations: Vec<Animation>,
}

#[derive(Serialize, Deserialize)]
pub struct Sprite {
    pub parts: Vec<SpritePart>,
}

#[derive(Serialize, Deserialize)]
pub struct SpritePart {
    pub texture_pos: PosInTexture,
    pub pos_x: i16,
    pub pos_y: i16,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub multiply_color: Color,
    pub screen_color: Color,
    pub opacity: u8,
    pub unk1: [u8; 12],
    pub designation_id: u8,
    pub unk2: u8,
    pub depth: StereoDepth,
}

#[derive(Serialize, Deserialize)]
pub struct PosInTexture {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Serialize, Deserialize)]
pub struct StereoDepth {
    pub top_left: f32,
    pub bottom_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
}

#[derive(Serialize, Deserialize)]
pub struct Animation {
    pub name: String, //Stored weirdly - do that manualy
    pub interpolation: i32,
    pub steps: Vec<AnimationStep>,
}

#[derive(Serialize, Deserialize)]
pub struct AnimationStep {
    pub sprite: u16,
    pub duration: u16,
    pub pos_x: i16,
    pub pos_y: i16,
    pub depth: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub rotation: f32,
    pub multiply_color: Color,
    pub unk: [u8; 3],
    pub opacity: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}

impl BCCAD {
    pub fn from_bccad(filename: &str) -> IOResult<Self> {
        let mut f = File::open(filename)?;
        let timestamp = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let texture_width = u16::read_from(&mut f, ByteOrder::LittleEndian)?;
        let texture_height = u16::read_from(&mut f, ByteOrder::LittleEndian)?;

        let sprite_count = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
        let mut sprites = vec![];
        for _ in 0..sprite_count {
            let parts_count = u32::read_from(&mut f, ByteOrder::LittleEndian)?;
            let mut parts = vec![];
            for _ in 0..parts_count {
                let texture_pos = PosInTexture {
                    x: u16::read_from(&mut f, ByteOrder::LittleEndian)?,
                    y: u16::read_from(&mut f, ByteOrder::LittleEndian)?,
                    width: u16::read_from(&mut f, ByteOrder::LittleEndian)?,
                    height: u16::read_from(&mut f, ByteOrder::LittleEndian)?,
                };
                let pos_x = i16::read_from(&mut f, ByteOrder::LittleEndian)?;
                let pos_y = i16::read_from(&mut f, ByteOrder::LittleEndian)?;
                let scale_x = f32::read_from(&mut f, ByteOrder::LittleEndian)?;
                let scale_y = f32::read_from(&mut f, ByteOrder::LittleEndian)?;
            }
            sprites.push(Sprite { parts });
        }

        Ok(Self {
            timestamp,
            texture_width,
            texture_height,
            sprites: vec![],
            animations: vec![],
        })
    }
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
