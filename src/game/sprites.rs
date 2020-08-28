use crate::game::digits::{load_digits, Digits};
use crate::game::helpers::get_image;
use ggez::error::GameResult;
use ggez::graphics::{FilterMode, Image};
use ggez::Context;

pub struct SpriteData {
    pub digits: Digits,
    pub dino: Image,
}

impl SpriteData {
    pub fn new(ctx: &mut Context, _scale: usize) -> GameResult<Self> {
        let sprite_data = include_bytes!("../../assets/sprites.png");

        let sprite_image =
            image::load_from_memory_with_format(sprite_data, image::ImageFormat::Png)
                .expect("Failed to load sprites");

        let mut dino_image = get_image(ctx, &sprite_image, [848, 2], [44, 47])?;
        dino_image.set_filter(FilterMode::Nearest);

        Ok(SpriteData {
            digits: load_digits(ctx, &sprite_image, [655, 2], [9, 11], 1)?,
            dino: dino_image,
        })
    }
}
