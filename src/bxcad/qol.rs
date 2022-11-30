use serde::{Deserialize, Serialize};

use crate::{
    bxcad::{bccad, brcad},
    BXCAD,
};
use std::collections::BTreeMap;

pub trait Indexizable: for<'de> BXCAD<'de> {
    type Indexized;
    fn to_indexized(self) -> Self::Indexized;
    fn from_indexized(og: Self::Indexized) -> Self;
}

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
        todo!();
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IndexizedBRCAD {
    pub timestamp: Option<u32>,
    pub texture_width: u16,
    pub texture_height: u16,
    pub sprites: BTreeMap<u16, brcad::Sprite>,
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
            texture_width: self.texture_width,
            texture_height: self.texture_height,
            sprites,
            animations: self.animations,
        }
    }

    fn from_indexized(og: Self::Indexized) -> Self {
        todo!();
    }
}