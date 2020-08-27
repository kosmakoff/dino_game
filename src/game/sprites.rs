use crate::game::digits::{load_digits, Digits};
use ggez::error::GameResult;
use ggez::Context;

pub struct SpriteData {
    pub digits: Digits,
}

impl SpriteData {
    pub fn new(ctx: &mut Context, _scale: usize) -> GameResult<Self> {
        let sprite_data = include_bytes!("../../assets/sprites.png");

        let sprite_image =
            image::load_from_memory_with_format(sprite_data, image::ImageFormat::Png)
                .expect("Failed to load sprites");

        Ok(SpriteData {
            digits: load_digits(ctx, &sprite_image, [655, 2], [9, 11], 1)?,
        })
    }
}
