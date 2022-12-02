use serde::{Deserialize, Serialize};

use crate::{
    bxcad::{bccad, brcad},
    BXCAD,
};
use std::collections::BTreeMap;

/// Trait for BXCAD types that allow "indexization", that is, conversion for their `Sprite` lists
/// from a Vec to some sort of Map (usually BTreeMap<uint, Sprite>)
pub trait Indexizable: for<'de> BXCAD<'de> {
    /// The type that contains indexized data for this BXCAD
    type Indexized;
    /// Convert a standard BXCAD to indexized
    fn to_indexized(self) -> Self::Indexized;
    /// Convert the indexized data to a standard BXCAD
    fn from_indexized(og: Self::Indexized) -> Self;
}

/// [Indexizable::Indexized] variant of [crate::BCCAD]
///
/// See that type for more information
#[derive(Serialize, Deserialize, Clone)]
pub struct IndexizedBCCAD {
    pub timestamp: Option<u32>,
    pub texture_width: u16,
    pub texture_height: u16,
    pub sprites: BTreeMap<u16, bccad::Sprite>,
    pub animations: Vec<bccad::Animation>,
}

impl Indexizable for bccad::BCCAD {
    type Indexized = IndexizedBCCAD;
    fn to_indexized(self) -> Self::Indexized {
        let mut sprites = BTreeMap::new();

        for (i, sprite) in self.sprites.iter().enumerate() {
            sprites.insert(i as u16, sprite.clone());
        }

        IndexizedBCCAD {
            timestamp: self.timestamp,
            texture_width: self.texture_width,
            texture_height: self.texture_height,
            sprites,
            animations: self.animations,
        }
    }

    fn from_indexized(og: Self::Indexized) -> Self {
        let mut sprites = vec![];

        for i in 0..=*og.sprites.keys().max().unwrap() {
            let sprite = match og.sprites.get(&i) {
                Some(c) => c.clone(),
                None => bccad::Sprite { parts: vec![] },
            };
            sprites.push(sprite);
        }

        Self {
            timestamp: og.timestamp,
            texture_width: og.texture_width,
            texture_height: og.texture_height,
            sprites,
            animations: og.animations,
        }
    }
}

/// [Indexizable::Indexized] variant of [crate::BRCAD]
///
/// See that type for more information
#[derive(Serialize, Deserialize, Clone)]
pub struct IndexizedBRCAD {
    pub timestamp: Option<u32>,
    pub unk0: u32,
    pub spritesheet_num: u16,
    pub spritesheet_control: u16,
    pub texture_width: u16,
    pub texture_height: u16,
    pub unk1: u16,
    pub sprites: BTreeMap<u16, brcad::Sprite>,
    pub unk2: u16,
    pub animations: Vec<brcad::Animation>,
}

impl Indexizable for brcad::BRCAD {
    type Indexized = IndexizedBRCAD;
    fn to_indexized(self) -> Self::Indexized {
        let mut sprites = BTreeMap::new();

        for (i, sprite) in self.sprites.iter().enumerate() {
            sprites.insert(i as u16, sprite.clone());
        }

        IndexizedBRCAD {
            timestamp: self.timestamp,
            unk0: self.unk0,
            spritesheet_num: self.spritesheet_num,
            spritesheet_control: self.spritesheet_control,
            texture_width: self.texture_width,
            texture_height: self.texture_height,
            unk1: self.unk1,
            sprites,
            unk2: self.unk2,
            animations: self.animations,
        }
    }

    fn from_indexized(og: Self::Indexized) -> Self {
        let mut sprites = vec![];

        for i in 0..=*og.sprites.keys().max().unwrap() {
            let sprite = match og.sprites.get(&i) {
                Some(c) => c.clone(),
                None => brcad::Sprite {
                    parts: vec![],
                    unk: 0,
                },
            };
            sprites.push(sprite);
        }

        Self {
            timestamp: og.timestamp,
            unk0: og.unk0,
            spritesheet_num: og.spritesheet_num,
            spritesheet_control: og.spritesheet_control,
            texture_width: og.texture_width,
            texture_height: og.texture_height,
            unk1: og.unk1,
            sprites,
            unk2: og.unk2,
            animations: og.animations,
        }
    }
}
