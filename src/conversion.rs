use crate::{
    bxcad::{
        bccad::{self, StereoDepth},
        brcad,
    },
    Color, BCCAD, BRCAD,
};

pub fn rcad_to_ccad(og: &BRCAD, scale_texture: bool) -> BCCAD {
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
                        texture_pos: part.texture_pos.clone(),

                        pos_x: part.pos_x as i16,
                        pos_y: part.pos_y as i16,
                        scale_x: part.scale_x,
                        scale_y: part.scale_y,
                        rotation: part.rotation,
                        flip_x: part.flip_x,
                        flip_y: part.flip_y,
                        multiply_color: Color::new(255, 255, 255),
                        screen_color: Color::new(0, 0, 0),
                        opacity: part.opacity,

                        depth: StereoDepth::none(),

                        // TODO: figure out proper starting values for these
                        unk1: [0; 12],
                        designation_id: 0,
                        unk2: 0,
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

                        pos_x: step.pos_x(),
                        pos_y: step.pos_y(),
                        scale_x: step.scale_x,
                        scale_y: step.scale_y,
                        rotation: step.rotation,
                        opacity: (step.opacity as u16),

                        multiply_color: Color::new(255, 255, 255),
                        depth: 0.0,

                        // TODO: figure out proper starting values for this
                        unk: [0; 3],
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

                // Origin is at (512, 512)
                part.pos_x = (part.pos_x - 512) / 2 + 512;
                part.pos_y = (part.pos_y - 512) / 2 + 512;
            }
        }
    }

    out
}

pub fn ccad_to_rcad(og: &BCCAD, scale_texture: bool) -> BRCAD {
    #[allow(deprecated)]
    let mut out = BRCAD {
        timestamp: None,
        texture_width: og.texture_width,
        texture_height: og.texture_height,
        sprites: og
            .sprites
            .iter()
            .map(|c| brcad::Sprite {
                unk: 0,
                parts: c
                    .parts
                    .iter()
                    .map(|part| brcad::SpritePart {
                        texture_pos: part.texture_pos.clone(),
                        pos_x: part.pos_x as u16,
                        pos_y: part.pos_y as u16,
                        scale_x: part.scale_x,
                        scale_y: part.scale_y,
                        rotation: part.rotation,
                        flip_x: part.flip_x,
                        flip_y: part.flip_y,
                        opacity: part.opacity,

                        // TODO: flock step birds_i will need this set
                        unk: 0,
                    })
                    .collect(),
            })
            .collect(),
        animations: og
            .animations
            .iter()
            .map(|c| brcad::Animation {
                name: Some(c.name.clone()),
                unk: 0,
                steps: c
                    .steps
                    .iter()
                    .map(|step| brcad::AnimationStep {
                        sprite: step.sprite,
                        duration: step.duration,
                        scale_x: step.scale_x,
                        scale_y: step.scale_y,
                        rotation: step.rotation,
                        opacity: step.opacity as u8,
                        unk0: 0,
                        unk1: [0; 3],
                    })
                    .collect(),
            })
            .collect(),

        unk1: 0,
        unk2: 0,

        // TODO: figure out good values for these
        spritesheet_num: 0,
        spritesheet_control: 0,

        // TODO: flock step birds_i will need this set
        unk0: 0,
    };

    for (i, animation) in out.animations.iter_mut().enumerate() {
        for (j, step) in animation.steps.iter_mut().enumerate() {
            *step.pos_x_mut() = og.animations[i].steps[j].pos_x;
            *step.pos_y_mut() = og.animations[i].steps[j].pos_y;
        }
    }

    if scale_texture {
        out.texture_width *= 2;
        out.texture_height *= 2;

        for sprite in &mut out.sprites {
            for part in &mut sprite.parts {
                part.texture_pos.x *= 2;
                part.texture_pos.y *= 2;
                part.texture_pos.width *= 2;
                part.texture_pos.height *= 2;

                part.pos_x = (part.pos_x - 512) * 2 + 512;
                part.pos_y = (part.pos_y - 512) * 2 + 512;
            }
        }
    }

    out
}
