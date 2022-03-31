use crate::{
    bxcad::{PosInTexture, BXCAD},
    bytestream_addon::ByteStream,
    Color, VarLenString,
};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use serde::{Deserialize, Serialize};
use std::io::{Read, Result as IOResult, Seek, Write};

#[derive(Serialize, Deserialize)]
pub struct BCCAD {
    pub timestamp: Option<u32>,
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
    pub flip_x: bool,
    pub flip_y: bool,
    pub multiply_color: Color,
    pub screen_color: Color,
    pub opacity: u8,
    pub unk1: [u8; 12],
    pub designation_id: u8,
    pub unk2: u8,
    pub depth: StereoDepth,
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
    pub name: String,
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

impl BXCAD<'_> for BCCAD {
    const BYTE_ORDER: ByteOrder = ByteOrder::LittleEndian;
    const TIMESTAMP: u32 = 20131007;
    fn from_binary<F: Read>(f: &mut F) -> IOResult<Self> {
        let timestamp = u32::read_from(f, Self::BYTE_ORDER)?;
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
                let pos_x = i16::read_from(f, Self::BYTE_ORDER)?;
                let pos_y = i16::read_from(f, Self::BYTE_ORDER)?;
                let scale_x = f32::read_from(f, Self::BYTE_ORDER)?;
                let scale_y = f32::read_from(f, Self::BYTE_ORDER)?;
                let rotation = f32::read_from(f, Self::BYTE_ORDER)?;
                let flip_x = bool::read_from(f, Self::BYTE_ORDER)?;
                let flip_y = bool::read_from(f, Self::BYTE_ORDER)?;
                let multiply_color = Color::read_from(f, Self::BYTE_ORDER)?;
                let screen_color = Color::read_from(f, Self::BYTE_ORDER)?;
                let opacity = u8::read_from(f, Self::BYTE_ORDER)?;
                let mut unk1 = [0; 12];
                f.read(&mut unk1)?;
                let designation_id = u8::read_from(f, Self::BYTE_ORDER)?;
                let unk2 = u8::read_from(f, Self::BYTE_ORDER)?;
                let depth = StereoDepth {
                    top_left: f32::read_from(f, Self::BYTE_ORDER)?,
                    bottom_left: f32::read_from(f, Self::BYTE_ORDER)?,
                    top_right: f32::read_from(f, Self::BYTE_ORDER)?,
                    bottom_right: f32::read_from(f, Self::BYTE_ORDER)?,
                };
                u8::read_from(f, Self::BYTE_ORDER)?; // terminator
                parts.push(SpritePart {
                    texture_pos,
                    pos_x,
                    pos_y,
                    scale_x,
                    scale_y,
                    rotation,
                    flip_x,
                    flip_y,
                    multiply_color,
                    screen_color,
                    opacity,
                    unk1,
                    designation_id,
                    unk2,
                    depth,
                })
            }
            sprites.push(Sprite { parts });
        }

        let anim_count = u32::read_from(f, Self::BYTE_ORDER)?;
        let mut animations = vec![];
        for _ in 0..anim_count {
            let name = VarLenString::read_from(f, Self::BYTE_ORDER)?.0.clone();
            let interpolation = i32::read_from(f, Self::BYTE_ORDER)?;
            let step_count = u32::read_from(f, Self::BYTE_ORDER)?;
            let mut steps = vec![];
            for _ in 0..step_count {
                let sprite = u16::read_from(f, Self::BYTE_ORDER)?;
                let duration = u16::read_from(f, Self::BYTE_ORDER)?;
                let pos_x = i16::read_from(f, Self::BYTE_ORDER)?;
                let pos_y = i16::read_from(f, Self::BYTE_ORDER)?;
                let depth = f32::read_from(f, Self::BYTE_ORDER)?;
                let scale_x = f32::read_from(f, Self::BYTE_ORDER)?;
                let scale_y = f32::read_from(f, Self::BYTE_ORDER)?;
                let rotation = f32::read_from(f, Self::BYTE_ORDER)?;
                let multiply_color = Color::read_from(f, Self::BYTE_ORDER)?;
                let mut unk = [0; 3];
                f.read(&mut unk)?;
                let opacity = u16::read_from(f, Self::BYTE_ORDER)?;
                steps.push(AnimationStep {
                    sprite,
                    duration,
                    pos_x,
                    pos_y,
                    depth,
                    scale_x,
                    scale_y,
                    rotation,
                    multiply_color,
                    unk,
                    opacity,
                })
            }
            animations.push(Animation {
                name,
                interpolation,
                steps,
            })
        }

        let timestamp = match timestamp {
            Self::TIMESTAMP => None,
            _ => Some(timestamp),
        };

        Ok(Self {
            timestamp,
            texture_width,
            texture_height,
            sprites,
            animations,
        })
    }
    fn to_binary<F: Write>(&self, f: &mut F) -> IOResult<()> {
        self.timestamp
            .unwrap_or(Self::TIMESTAMP)
            .write_to(f, Self::BYTE_ORDER)?;
        self.texture_width.write_to(f, Self::BYTE_ORDER)?;
        self.texture_height.write_to(f, Self::BYTE_ORDER)?;

        (self.sprites.len() as u32).write_to(f, Self::BYTE_ORDER)?;
        for sprite in &self.sprites {
            (sprite.parts.len() as u32).write_to(f, Self::BYTE_ORDER)?;
            for part in &sprite.parts {
                part.texture_pos.x.write_to(f, Self::BYTE_ORDER)?;
                part.texture_pos.y.write_to(f, Self::BYTE_ORDER)?;
                part.texture_pos.width.write_to(f, Self::BYTE_ORDER)?;
                part.texture_pos.height.write_to(f, Self::BYTE_ORDER)?;
                part.pos_x.write_to(f, Self::BYTE_ORDER)?;
                part.pos_y.write_to(f, Self::BYTE_ORDER)?;
                part.scale_x.write_to(f, Self::BYTE_ORDER)?;
                part.scale_y.write_to(f, Self::BYTE_ORDER)?;
                part.rotation.write_to(f, Self::BYTE_ORDER)?;
                part.flip_x.write_to(f, Self::BYTE_ORDER)?;
                part.flip_y.write_to(f, Self::BYTE_ORDER)?;
                part.multiply_color.write_to(f, Self::BYTE_ORDER)?;
                part.screen_color.write_to(f, Self::BYTE_ORDER)?;
                part.opacity.write_to(f, Self::BYTE_ORDER)?;
                f.write(&part.unk1)?;
                part.designation_id.write_to(f, Self::BYTE_ORDER)?;
                part.unk2.write_to(f, Self::BYTE_ORDER)?;
                part.depth.top_left.write_to(f, Self::BYTE_ORDER)?;
                part.depth.bottom_left.write_to(f, Self::BYTE_ORDER)?;
                part.depth.top_right.write_to(f, Self::BYTE_ORDER)?;
                part.depth.bottom_right.write_to(f, Self::BYTE_ORDER)?;
                (0 as u8).write_to(f, Self::BYTE_ORDER)?; // terminator
            }
        }

        (self.animations.len() as u32).write_to(f, Self::BYTE_ORDER)?;
        for anim in &self.animations {
            VarLenString(anim.name.clone()).write_to(f, Self::BYTE_ORDER)?;
            anim.interpolation.write_to(f, Self::BYTE_ORDER)?;
            (anim.steps.len() as u32).write_to(f, Self::BYTE_ORDER)?;
            for step in &anim.steps {
                step.sprite.write_to(f, Self::BYTE_ORDER)?;
                step.duration.write_to(f, Self::BYTE_ORDER)?;
                step.pos_x.write_to(f, Self::BYTE_ORDER)?;
                step.pos_y.write_to(f, Self::BYTE_ORDER)?;
                step.depth.write_to(f, Self::BYTE_ORDER)?;
                step.scale_x.write_to(f, Self::BYTE_ORDER)?;
                step.scale_y.write_to(f, Self::BYTE_ORDER)?;
                step.rotation.write_to(f, Self::BYTE_ORDER)?;
                step.multiply_color.write_to(f, Self::BYTE_ORDER)?;
                f.write(&step.unk)?;
                step.opacity.write_to(f, Self::BYTE_ORDER)?;
            }
        }
        (0 as u8).write_to(f, Self::BYTE_ORDER)?; // terminator

        Ok(())
    }
}
impl BCCAD {
    #[deprecated]
    pub fn from_bccad<F: Read + Seek>(f: &mut F) -> IOResult<Self> {
        Self::from_binary(f)
    }
    #[deprecated]
    pub fn to_bccad<F: Write>(&self, f: &mut F) -> IOResult<()> {
        self.to_binary(f)
    }
    #[deprecated]
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    #[deprecated]
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
