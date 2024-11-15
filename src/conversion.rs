use crate::{
    bxcad::{
        bccad::{self, StereoDepth},
        brcad, PosInTexture,
    },
    Color, BCCAD, BRCAD, BXCAD,
};

pub fn ccad_to_rcad(og: &BRCAD, scale_texture: bool) -> BCCAD {
    let mut out = BCCAD {
        timestamp: None,
        texture_width: og.texture_width,
        texture_height: og.texture_height,
        sprites: og
            .sprites
            .iter()
            .map(|c| bccad::Sprite {
                parts: c
                    .parts
                    .iter()
                    .map(|part| bccad::SpritePart {
                        texture_pos: PosInTexture {
                            x: part.texture_pos.x,
                            y: part.texture_pos.y,
                            width: part.texture_pos.width,
                            height: part.texture_pos.height,
                        },

                        // Origin is at (512, 512)
                        pos_x: (part.pos_x as i16 - 512) / 2 + 512,
                        pos_y: (part.pos_y as i16 - 512) / 2 + 512,

                        scale_x: part.scale_x / 2.0,
                        scale_y: part.scale_y / 2.0,
                        rotation: part.rotation,
                        flip_x: part.flip_x,
                        flip_y: part.flip_y,
                        multiply_color: Color::new(255, 255, 255),
                        screen_color: Color::new(0, 0, 0),
                        opacity: part.opacity,

                        // TODO: figure out proper starting values for these
                        unk1: [0; 12],
                        designation_id: 0,
                        unk2: 0,

                        depth: StereoDepth::none(),
                    })
                    .collect(),
            })
            .collect(),
        animations: og
            .animations
            .iter()
            .enumerate()
            .map(|(c, anim)| bccad::Animation {
                name: anim.name.clone().unwrap_or(format!("anim_{}", c)),
                interpolation: 0,
                steps: anim
                    .steps
                    .iter()
                    .map(|step| bccad::AnimationStep {
                        sprite: step.sprite,
                        duration: step.duration,
                        pos_x: 0,
                        pos_y: 0,
                        ..todo!()
                    })
                    .collect(),
            })
            .collect(),
    };

    if scale_texture {
        out.texture_width /= 2;
        out.texture_height /= 2;

        for sprite in &mut out.sprites {
            for part in &mut sprite.parts {
                part.texture_pos.x /= 2;
                part.texture_pos.y /= 2;
                part.texture_pos.width /= 2;
                part.texture_pos.height /= 2;
            }
        }
    }

    out
}
