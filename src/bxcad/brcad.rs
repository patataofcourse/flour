use crate::{
    bxcad::{BXCADType, PosInTexture, BXCAD},
    bytestream_addon::ByteStream,
    error::{Error, Result},
};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use encoding_rs::SHIFT_JIS;
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, Write};

/// A representation of the contents of a BRCAD file
#[derive(Serialize, Deserialize, Clone)]
pub struct BRCAD {
    /// Date of latest format revision, in YYYYMMDD format (decimal).
    /// Known timestamp is 20100312 (Mar 12 2010)
    pub timestamp: Option<u32>,
    pub unk0: u32,
    /// Number of the spritesheet to use in a specific
    pub spritesheet_num: u16,
    /// Might be a BOM?
    pub spritesheet_control: u16,
    /// Width of the associated texture in pixels
    pub texture_width: u16,
    /// Height of the associated texture in pixels
    pub texture_height: u16,
    pub unk1: u16,
    /// [`Sprite`]s this BRCAD contains
    pub sprites: Vec<Sprite>,
    pub unk2: u16,
    /// [`Animation`]s that can be called for this BRCAD
    pub animations: Vec<Animation>,
}

/// Struct that represents a labels file for a BRCAD file - just a bunch of #defines
#[derive(Serialize, Deserialize, Clone)]
pub struct BRCADLabels(pub Vec<String>);

/// A frame of a BRCAD animation, composed of several [`SpritePart`]s or cells
/// aligned together to create a full picture
#[derive(Serialize, Deserialize, Clone)]
pub struct Sprite {
    pub unk: u16,
    /// [`SpritePart`]s that form this Sprite
    pub parts: Vec<SpritePart>,
}

/// A small image taken directly from the texture sheet, which grouped with others
/// creates a full frame of the animation, that is, a [`Sprite`]
#[derive(Serialize, Deserialize, Clone)]
pub struct SpritePart {
    /// Struct that defines the bounds of the SpritePart in the texture itself
    pub texture_pos: PosInTexture,
    pub unk: u32,
    /// X position where the part should be placed relative to the sprite
    pub pos_x: u16,
    /// Y position where the part should be placed relative to the sprite
    pub pos_y: u16,
    /// Scaling factor for the X axis
    pub scale_x: f32,
    /// Scaling factor for the Y axis
    pub scale_y: f32,
    /// Part rotation in degrees
    pub rotation: f32,
    /// Whether to flip the part on the X axis
    pub flip_x: bool,
    /// Whether to flip the part on the Y axis
    pub flip_y: bool,
    /// Opacity for the part
    pub opacity: u8,
}

/// A cell animation for BRCAD, composed of different frames/[`Sprite`]s
#[derive(Serialize, Deserialize, Clone)]
pub struct Animation {
    /// The name of the animation. Defined in the labels file, this may be
    /// missing from the struct if said file is not provided. **Do NOT change
    /// the order of the animations, the name is just there for development
    /// purposes**
    pub name: Option<String>,
    pub unk: u16,
    /// List of [`AnimationStep`]s that constitute this Animation
    pub steps: Vec<AnimationStep>,
}

/// These constitute an [`Animation`], and are a reference to
/// a [`Sprite`] plus more information about it relative to the
/// whole animation
#[derive(Serialize, Deserialize, Clone)]
pub struct AnimationStep {
    /// A reference to the index number of the [`Sprite`] this AnimationStep uses
    pub sprite: u16,
    /// Duration of the frame (in unknown units, seems to have changeable speed)
    pub duration: u16,
    pub unk0: u32,
    /// Scaling factor for the X axis
    pub scale_x: f32,
    /// Scaling factor for the Y axis
    pub scale_y: f32,
    /// Rotation in degrees
    pub rotation: f32,
    /// Opacity for the sprite
    pub opacity: u8,
    pub unk1: [u8; 3],
}

impl BXCAD<'_> for BRCAD {
    const BYTE_ORDER: ByteOrder = ByteOrder::BigEndian;
    const TIMESTAMP: u32 = 20100312;
    const BXCAD_TYPE: BXCADType = BXCADType::BRCAD;
    fn from_binary<F: Read + Seek>(f: &mut F) -> Result<Self> {
        let timestamp = u32::read_from(f, Self::BYTE_ORDER)?;
        let unk0 = u32::read_from(f, Self::BYTE_ORDER)?;
        let spritesheet_num = u16::read_from(f, Self::BYTE_ORDER)?;
        let spritesheet_control = u16::read_from(f, Self::BYTE_ORDER)?;
        let texture_width = u16::read_from(f, Self::BYTE_ORDER)?;
        let texture_height = u16::read_from(f, Self::BYTE_ORDER)?;

        let sprite_count = u16::read_from(f, Self::BYTE_ORDER)?;
        let unk1 = u16::read_from(f, Self::BYTE_ORDER)?; //unknown
        let mut sprites = vec![];
        for _ in 0..sprite_count {
            let parts_count = u16::read_from(f, Self::BYTE_ORDER)?;
            let unk = u16::read_from(f, Self::BYTE_ORDER)?; //unknown
            let mut parts = vec![];
            for _ in 0..parts_count {
                let texture_pos = PosInTexture {
                    x: u16::read_from(f, Self::BYTE_ORDER)?,
                    y: u16::read_from(f, Self::BYTE_ORDER)?,
                    width: u16::read_from(f, Self::BYTE_ORDER)?,
                    height: u16::read_from(f, Self::BYTE_ORDER)?,
                };
                let unk = u32::read_from(f, Self::BYTE_ORDER)?;
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
                    unk,
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
            sprites.push(Sprite { unk, parts });
        }
        let animation_count = u16::read_from(f, Self::BYTE_ORDER)?;
        let unk2 = u16::read_from(f, Self::BYTE_ORDER)?; //unknown
        let mut animations = vec![];
        for _ in 0..animation_count {
            let step_count = u16::read_from(f, Self::BYTE_ORDER)?;
            let unk = u16::read_from(f, Self::BYTE_ORDER)?; //unknown
            let mut steps = vec![];
            for _ in 0..step_count {
                let sprite = u16::read_from(f, Self::BYTE_ORDER)?;
                let duration = u16::read_from(f, Self::BYTE_ORDER)?;
                let unk0 = u32::read_from(f, Self::BYTE_ORDER)?;
                let scale_x = f32::read_from(f, Self::BYTE_ORDER)?;
                let scale_y = f32::read_from(f, Self::BYTE_ORDER)?;
                let rotation = f32::read_from(f, Self::BYTE_ORDER)?;
                let opacity = u8::read_from(f, Self::BYTE_ORDER)?;
                let mut unk1 = [0u8; 3];
                f.read(&mut unk1)?;

                steps.push(AnimationStep {
                    sprite,
                    duration,
                    unk0,
                    scale_x,
                    scale_y,
                    rotation,
                    opacity,
                    unk1,
                });
            }

            animations.push(Animation {
                name: None,
                unk,
                steps,
            });
        }
        let timestamp = match timestamp {
            Self::TIMESTAMP => None,
            _ => Some(timestamp),
        };

        Ok(BRCAD {
            timestamp,
            unk0,
            spritesheet_num,
            spritesheet_control,
            texture_width,
            texture_height,
            unk1,
            sprites,
            unk2,
            animations,
        })
    }
    fn to_binary<F: Write>(&self, f: &mut F) -> Result<()> {
        self.timestamp
            .unwrap_or(Self::TIMESTAMP)
            .write_to(f, Self::BYTE_ORDER)?;
        self.unk0.write_to(f, Self::BYTE_ORDER)?;
        self.spritesheet_num.write_to(f, Self::BYTE_ORDER)?;
        self.spritesheet_control.write_to(f, Self::BYTE_ORDER)?;
        self.texture_width.write_to(f, Self::BYTE_ORDER)?;
        self.texture_height.write_to(f, Self::BYTE_ORDER)?;

        (self.sprites.len() as u16).write_to(f, Self::BYTE_ORDER)?;
        self.unk1.write_to(f, Self::BYTE_ORDER)?;
        for sprite in &self.sprites {
            (sprite.parts.len() as u16).write_to(f, Self::BYTE_ORDER)?;
            sprite.unk.write_to(f, Self::BYTE_ORDER)?;
            for part in &sprite.parts {
                part.texture_pos.x.write_to(f, Self::BYTE_ORDER)?;
                part.texture_pos.y.write_to(f, Self::BYTE_ORDER)?;
                part.texture_pos.width.write_to(f, Self::BYTE_ORDER)?;
                part.texture_pos.height.write_to(f, Self::BYTE_ORDER)?;
                part.unk.write_to(f, Self::BYTE_ORDER)?;
                part.pos_x.write_to(f, Self::BYTE_ORDER)?;
                part.pos_y.write_to(f, Self::BYTE_ORDER)?;
                part.scale_x.write_to(f, Self::BYTE_ORDER)?;
                part.scale_y.write_to(f, Self::BYTE_ORDER)?;
                part.rotation.write_to(f, Self::BYTE_ORDER)?;
                part.flip_x.write_to(f, Self::BYTE_ORDER)?;
                part.flip_y.write_to(f, Self::BYTE_ORDER)?;
                part.opacity.write_to(f, Self::BYTE_ORDER)?;
                (0 as u8).write_to(f, Self::BYTE_ORDER)?; // terminator/padding
            }
        }

        (self.animations.len() as u16).write_to(f, Self::BYTE_ORDER)?;
        self.unk2.write_to(f, Self::BYTE_ORDER)?;
        for anim in &self.animations {
            (anim.steps.len() as u16).write_to(f, Self::BYTE_ORDER)?;
            anim.unk.write_to(f, Self::BYTE_ORDER)?;
            for step in &anim.steps {
                step.sprite.write_to(f, Self::BYTE_ORDER)?;
                step.duration.write_to(f, Self::BYTE_ORDER)?;
                step.unk0.write_to(f, Self::BYTE_ORDER)?;
                step.scale_x.write_to(f, Self::BYTE_ORDER)?;
                step.scale_y.write_to(f, Self::BYTE_ORDER)?;
                step.rotation.write_to(f, Self::BYTE_ORDER)?;
                step.opacity.write_to(f, Self::BYTE_ORDER)?;
                f.write(&step.unk1)?;
            }
        }
        Ok(())
    }
}

impl BRCAD {
    /// Uses the contents of the associated labels file to add names to the struct
    pub fn apply_labels<F: Read>(&mut self, labels: &mut F) -> Result<()> {
        let mut data = vec![];
        labels.read_to_end(&mut data)?;
        let (labdata, _, errors) = SHIFT_JIS.decode(&data);
        if errors {
            Err(Error::LabelsFileNotShiftJIS)?
        }
        for line in labdata.lines() {
            let line = line
                .split_once("//")
                .unwrap_or((line, ""))
                .0
                .replace("\t", " ");
            if line.starts_with("#define ") {
                // fuck spacing so much
                let line = line
                    .split_once(" ")
                    .unwrap()
                    .1
                    .trim()
                    .split_once(" ")
                    .unwrap();
                let num = match line
                    .1
                    .trim()
                    .split_once(" ")
                    .unwrap_or((line.1, ""))
                    .0
                    .parse::<usize>()
                {
                    Ok(c) => c,
                    Err(_) => Err(Error::BadLabelsFile)?,
                };

                match self.animations.get_mut(num) {
                    Some(c) => c.name = Some(line.0.to_string()),
                    None => Err(Error::BadLabelsFile)?,
                }
            }
        }
        Ok(())
    }
}
