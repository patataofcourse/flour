use serde_derive::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct BCCAD {
    pub timestamp: u32,
    pub texture_width: u16,
    pub texture_height: u16,
    pub sprites: Vec<Sprite>,
    pub animations: Vec<Animation>
}

#[derive(Serialize, Deserialize)]
struct Sprite {
    pub parts: Vec<SpritePart>,
}

#[derive(Serialize, Deserialize)]
struct SpritePart {
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
    pub depth: StereoDepth
}

#[derive(Serialize, Deserialize)]
struct PosInTexture {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

#[derive(Serialize, Deserialize)]
struct StereoDepth {
    pub top_left: f32,
    pub bottom_left: f32,
    pub top_right: f32,
    pub bottom_right: f32,
}

#[derive(Serialize, Deserialize)]
struct Animation {
    pub name: String, //Stored weirdly - do that manualy
    pub interpolation: i32,
    pub steps: Vec<AnimationStep>,
}

#[derive(Serialize, Deserialize)]
struct AnimationStep {
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
struct Color {
    pub red: u8,
    pub blue: u8,
    pub green: u8,
}