use crate::{bytestream_addon::ByteStream, Color, VarLenString};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use serde_derive::{Deserialize, Serialize};
use std::{
    io::{Read, Result as IOResult, Write},
    marker::Sized,
};

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

impl BCCAD {
    pub fn from_bccad<F: Read>(f: &mut F) -> IOResult<Self>
    where
        F: Sized,
    {
        let timestamp = u32::read_from(f, ByteOrder::LittleEndian)?;
        let texture_width = u16::read_from(f, ByteOrder::LittleEndian)?;
        let texture_height = u16::read_from(f, ByteOrder::LittleEndian)?;

        let sprite_count = u32::read_from(f, ByteOrder::LittleEndian)?;
        let mut sprites = vec![];
        for _ in 0..sprite_count {
            let parts_count = u32::read_from(f, ByteOrder::LittleEndian)?;
            let mut parts = vec![];
            for _ in 0..parts_count {
                let texture_pos = PosInTexture {
                    x: u16::read_from(f, ByteOrder::LittleEndian)?,
                    y: u16::read_from(f, ByteOrder::LittleEndian)?,
                    width: u16::read_from(f, ByteOrder::LittleEndian)?,
                    height: u16::read_from(f, ByteOrder::LittleEndian)?,
                };
                let pos_x = i16::read_from(f, ByteOrder::LittleEndian)?;
                let pos_y = i16::read_from(f, ByteOrder::LittleEndian)?;
                let scale_x = f32::read_from(f, ByteOrder::LittleEndian)?;
                let scale_y = f32::read_from(f, ByteOrder::LittleEndian)?;
                let rotation = f32::read_from(f, ByteOrder::LittleEndian)?;
                let flip_x = bool::read_from(f, ByteOrder::LittleEndian)?;
                let flip_y = bool::read_from(f, ByteOrder::LittleEndian)?;
                let multiply_color = Color::read_from(f, ByteOrder::LittleEndian)?;
                let screen_color = Color::read_from(f, ByteOrder::LittleEndian)?;
                let opacity = u8::read_from(f, ByteOrder::LittleEndian)?;
                let mut unk1 = [0; 12];
                f.read(&mut unk1)?;
                let designation_id = u8::read_from(f, ByteOrder::LittleEndian)?;
                let unk2 = u8::read_from(f, ByteOrder::LittleEndian)?;
                let depth = StereoDepth {
                    top_left: f32::read_from(f, ByteOrder::LittleEndian)?,
                    bottom_left: f32::read_from(f, ByteOrder::LittleEndian)?,
                    top_right: f32::read_from(f, ByteOrder::LittleEndian)?,
                    bottom_right: f32::read_from(f, ByteOrder::LittleEndian)?,
                };
                u8::read_from(f, ByteOrder::LittleEndian)?; // terminator
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

        let anim_count = u32::read_from(f, ByteOrder::LittleEndian)?;
        let mut animations = vec![];
        for _ in 0..anim_count {
            let name = VarLenString::read_from(f, ByteOrder::LittleEndian)?
                .0
                .clone();
            let interpolation = i32::read_from(f, ByteOrder::LittleEndian)?;
            let step_count = u32::read_from(f, ByteOrder::LittleEndian)?;
            let mut steps = vec![];
            for _ in 0..step_count {
                let sprite = u16::read_from(f, ByteOrder::LittleEndian)?;
                let duration = u16::read_from(f, ByteOrder::LittleEndian)?;
                let pos_x = i16::read_from(f, ByteOrder::LittleEndian)?;
                let pos_y = i16::read_from(f, ByteOrder::LittleEndian)?;
                let depth = f32::read_from(f, ByteOrder::LittleEndian)?;
                let scale_x = f32::read_from(f, ByteOrder::LittleEndian)?;
                let scale_y = f32::read_from(f, ByteOrder::LittleEndian)?;
                let rotation = f32::read_from(f, ByteOrder::LittleEndian)?;
                let multiply_color = Color::read_from(f, ByteOrder::LittleEndian)?;
                let mut unk = [0; 3];
                f.read(&mut unk)?;
                let opacity = u16::read_from(f, ByteOrder::LittleEndian)?;
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

        Ok(Self {
            timestamp,
            texture_width,
            texture_height,
            sprites,
            animations,
        })
    }
    pub fn to_bccad<F: Write>(&self, f: &mut F) -> IOResult<()> {
        self.timestamp.write_to(f, ByteOrder::LittleEndian)?;
        self.texture_width.write_to(f, ByteOrder::LittleEndian)?;
        self.texture_height.write_to(f, ByteOrder::LittleEndian)?;

        (self.sprites.len() as u32).write_to(f, ByteOrder::LittleEndian)?;
        for sprite in &self.sprites {
            (sprite.parts.len() as u32).write_to(f, ByteOrder::LittleEndian)?;
            for part in &sprite.parts {
                part.texture_pos.x.write_to(f, ByteOrder::LittleEndian)?;
                part.texture_pos.y.write_to(f, ByteOrder::LittleEndian)?;
                part.texture_pos
                    .width
                    .write_to(f, ByteOrder::LittleEndian)?;
                part.texture_pos
                    .height
                    .write_to(f, ByteOrder::LittleEndian)?;
                part.pos_x.write_to(f, ByteOrder::LittleEndian)?;
                part.pos_y.write_to(f, ByteOrder::LittleEndian)?;
                part.scale_x.write_to(f, ByteOrder::LittleEndian)?;
                part.scale_y.write_to(f, ByteOrder::LittleEndian)?;
                part.rotation.write_to(f, ByteOrder::LittleEndian)?;
                part.flip_x.write_to(f, ByteOrder::LittleEndian)?;
                part.flip_y.write_to(f, ByteOrder::LittleEndian)?;
                part.multiply_color.write_to(f, ByteOrder::LittleEndian)?;
                part.screen_color.write_to(f, ByteOrder::LittleEndian)?;
                part.opacity.write_to(f, ByteOrder::LittleEndian)?;
                f.write(&part.unk1)?;
                part.designation_id.write_to(f, ByteOrder::LittleEndian)?;
                part.unk2.write_to(f, ByteOrder::LittleEndian)?;
                part.depth.top_left.write_to(f, ByteOrder::LittleEndian)?;
                part.depth
                    .bottom_left
                    .write_to(f, ByteOrder::LittleEndian)?;
                part.depth.top_right.write_to(f, ByteOrder::LittleEndian)?;
                part.depth
                    .bottom_right
                    .write_to(f, ByteOrder::LittleEndian)?;
                (0 as u8).write_to(f, ByteOrder::LittleEndian)?; // terminator
            }
        }

        (self.animations.len() as u32).write_to(f, ByteOrder::LittleEndian)?;
        for anim in &self.animations {
            VarLenString(anim.name.clone()).write_to(f, ByteOrder::LittleEndian)?;
            anim.interpolation.write_to(f, ByteOrder::LittleEndian)?;
            (anim.steps.len() as u32).write_to(f, ByteOrder::LittleEndian)?;
            for step in &anim.steps {
                step.sprite.write_to(f, ByteOrder::LittleEndian)?;
                step.duration.write_to(f, ByteOrder::LittleEndian)?;
                step.pos_x.write_to(f, ByteOrder::LittleEndian)?;
                step.pos_y.write_to(f, ByteOrder::LittleEndian)?;
                step.depth.write_to(f, ByteOrder::LittleEndian)?;
                step.scale_x.write_to(f, ByteOrder::LittleEndian)?;
                step.scale_y.write_to(f, ByteOrder::LittleEndian)?;
                step.rotation.write_to(f, ByteOrder::LittleEndian)?;
                step.multiply_color.write_to(f, ByteOrder::LittleEndian)?;
                f.write(&step.unk)?;
                step.opacity.write_to(f, ByteOrder::LittleEndian)?;
            }
        }
        (0 as u8).write_to(f, ByteOrder::LittleEndian)?; // terminator

        Ok(())
    }

    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
