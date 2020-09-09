use crate::game::helpers::get_image;
use ggez::error::GameResult;
use ggez::graphics::Image;
use ggez::Context;
use image::DynamicImage;

pub struct DinoData {
    pub idle: Image,
    pub blink: Image,
    pub walk1: Image,
    pub walk2: Image,
    pub crouched1: Image,
    pub crouched2: Image,
    pub hit: Image,
}

impl DinoData {
    pub fn new(
        ctx: &mut Context,
        sprite_sheet: &DynamicImage,
        normal_state_start: [usize; 2],
        normal_state_size: [usize; 2],
        crouched_state_start: [usize; 2],
        crouched_state_size: [usize; 2],
    ) -> GameResult<Self> {
        // 848,2 44,47 - first block

        let [normal_state_start_x, normal_state_start_y] = normal_state_start;
        let [normal_state_size_width, _] = normal_state_size;

        let image_idle = get_image(
            ctx,
            sprite_sheet,
            [normal_state_start_x, normal_state_start_y],
            normal_state_size,
        )?;

        let image_blink = get_image(
            ctx,
            sprite_sheet,
            [
                normal_state_start_x + normal_state_size_width,
                normal_state_start_y,
            ],
            normal_state_size,
        )?;

        let image_walk1 = get_image(
            ctx,
            sprite_sheet,
            [
                normal_state_start_x + normal_state_size_width * 2,
                normal_state_start_y,
            ],
            normal_state_size,
        )?;

        let image_walk2 = get_image(
            ctx,
            sprite_sheet,
            [
                normal_state_start_x + normal_state_size_width * 3,
                normal_state_start_y,
            ],
            normal_state_size,
        )?;

        let image_hit = get_image(
            ctx,
            sprite_sheet,
            [
                normal_state_start_x + normal_state_size_width * 3,
                normal_state_start_y,
            ],
            normal_state_size,
        )?;

        let [crouched_state_start_x, crouched_state_start_y] = crouched_state_start;
        let [crouched_state_size_width, _] = crouched_state_size;

        let image_crouched1 = get_image(
            ctx,
            sprite_sheet,
            [crouched_state_start_x, crouched_state_start_y],
            crouched_state_size,
        )?;

        let image_crouched2 = get_image(
            ctx,
            sprite_sheet,
            [
                crouched_state_start_x + crouched_state_size_width,
                crouched_state_start_y,
            ],
            crouched_state_size,
        )?;

        Ok(DinoData {
            idle: image_idle,
            blink: image_blink,
            walk1: image_walk1,
            walk2: image_walk2,
            crouched1: image_crouched1,
            crouched2: image_crouched2,
            hit: image_hit,
        })
    }
}
