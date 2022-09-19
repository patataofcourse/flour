use crate::{
    bxcad::{BXCADType, PosInTexture, BXCAD},
    bytestream_addon::ByteStream,
    error::Result,
    Color, VarLenString,
};
use bytestream::{ByteOrder, StreamReader, StreamWriter};
use serde::{Deserialize, Serialize};
use std::io::{Read, Seek, Write};

/// A representation of the contents of a BCCAD file
#[derive(Serialize, Deserialize)]
pub struct BCCAD {
    /// Date of latest format revision, in YYYYMMDD format (decimal).
    /// Known timestamp is 20131007 (Oct 7 2013)
    pub timestamp: Option<u32>,
    /// Width of the associated texture in pixels. 1024 maximum by hardware limitation
    pub texture_width: u16,
    /// Width of the associated texture in pixels. 1024 maximum by hardware limitation
    pub texture_height: u16,
    /// [`Sprite`]s this BCCAD contains
    pub sprites: Vec<Sprite>,
    /// [`Animation`]s that can be called for this BCCAD
    pub animations: Vec<Animation>,
}

/// A frame of a BCCAD animation, composed of several [`SpritePart`]s or cells
/// aligned together to create a full picture
#[derive(Serialize, Deserialize)]
pub struct Sprite {
    /// [`SpritePart`]s that form this Sprite
    pub parts: Vec<SpritePart>,
}

/// A small image taken directly from the texture sheet, which grouped with others
/// creates a full frame of the animation, that is, a [`Sprite`]
#[derive(Serialize, Deserialize)]
pub struct SpritePart {
    /// Struct that defines the bounds of the SpritePart in the texture itself
    pub texture_pos: PosInTexture,
    /// X position where the part should be placed relative to the sprite
    pub pos_x: i16,
    /// Y position where the part should be placed relative to the sprite
    pub pos_y: i16,
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
    /// A color to apply the multiply blending mode with for the
    /// part in order to darken or tint it
    pub multiply_color: Color,
    /// A color to apply the screen blending mode with for the
    /// part in order to lighten it
    pub screen_color: Color,
    /// Opacity for the part
    pub opacity: u8,
    pub unk1: [u8; 12],
    /// Seems to identify certain parts for the code to apply effects or other
    /// textures too, or use them as interactive pieces (see: Feed Goat game)
    pub designation_id: u8,
    pub unk2: u8,
    /// Stereoscopic depth for the part
    pub depth: StereoDepth,
}

/// Stereoscopic depth in the four corners of a rectangle (on which a [`SpritePart`]
/// is rendered)
#[derive(Serialize, Deserialize)]
pub struct StereoDepth {
    /// Top-left corner
    pub top_left: f32,
    /// Bottom-left corner
    pub bottom_left: f32,
    /// Top-right corner
    pub top_right: f32,
    /// Bottom-right corner
    pub bottom_right: f32,
}

/// A cell animation for BCCAD, composed of different frames/[`Sprite`]s
#[derive(Serialize, Deserialize)]
pub struct Animation {
    /// The name of the animation. This is what the game refers to it by
    pub name: String,
    /// Amount of interpolation used ???
    pub interpolation: i32,
    /// List of [`AnimationStep`]s that constitute this Animation
    pub steps: Vec<AnimationStep>,
}

/// These constitute an [`Animation`], and are a reference to
/// a [`Sprite`] plus more information about it relative to the
/// whole animation
#[derive(Serialize, Deserialize)]
pub struct AnimationStep {
    /// A reference to the index number of the [`Sprite`] this AnimationStep uses
    pub sprite: u16,
    /// Duration of the frame (in unknown units, seems to have changeable speed)
    pub duration: u16,
    /// X position the sprite is rendered to, relative to the animation
    pub pos_x: i16,
    /// Y position the sprite is rendered to, relative to the animation
    pub pos_y: i16,
    /// Stereoscopic depth applied to the whole [`Sprite`]
    pub depth: f32,
    /// Scaling factor for the X axis
    pub scale_x: f32,
    /// Scaling factor for the Y axis
    pub scale_y: f32,
    /// Rotation in degrees
    pub rotation: f32,
    /// A color to apply the multiply blending mode with for the
    /// [`Sprite`] in order to darken or tint it
    pub multiply_color: Color,
    pub unk: [u8; 3],
    /// Opacity for the sprite
    pub opacity: u16,
}

impl BXCAD<'_> for BCCAD {
    const BYTE_ORDER: ByteOrder = ByteOrder::LittleEndian;
    const TIMESTAMP: u32 = 20131007;
    const BXCAD_TYPE: BXCADType = BXCADType::BCCAD;
    fn from_binary<F: Read>(f: &mut F) -> Result<Self> {
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
    fn to_binary<F: Write>(&self, f: &mut F) -> Result<()> {
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
    #[deprecated(
        since = "2.0.0",
        note = "use `BCCAD::from_binary` (`BXCAD` trait) instead"
    )]
    pub fn from_bccad<F: Read + Seek>(f: &mut F) -> Result<Self> {
        Self::from_binary(f)
    }
    #[deprecated(
        since = "2.0.0",
        note = "use `BCCAD::to_binary` (`BXCAD` trait) instead"
    )]
    pub fn to_bccad<F: Write>(&self, f: &mut F) -> Result<()> {
        self.to_binary(f)
    }
    #[deprecated(since = "2.0.0", note = "use `::serde_json::from_str` instead")]
    pub fn from_json(json: &str) -> Result<Self> {
        Ok(serde_json::from_str(json)?)
    }
    #[deprecated(since = "2.0.0", note = "use `::serde_json::to_string_pretty` instead")]
    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}
