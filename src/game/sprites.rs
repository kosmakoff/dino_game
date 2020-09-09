use crate::game::sprites::digits::Digits;
use crate::game::sprites::dino::DinoData;
use crate::game::sprites::ground::Ground;
use ggez::error::GameResult;
use ggez::graphics::Image;
use ggez::Context;

mod digits;
mod dino;
mod ground;

pub struct SpriteData {
    pub digits: Digits,
    pub ground: Ground,
    pub dino: DinoData,
}

impl SpriteData {
    pub fn new(ctx: &mut Context, _scale: usize) -> GameResult<Self> {
        let sprite_data = include_bytes!("../../assets/sprites.png");

        let sprite_image =
            image::load_from_memory_with_format(sprite_data, image::ImageFormat::Png)
                .expect("Failed to load sprites");

        Ok(SpriteData {
            digits: Digits::new(ctx, &sprite_image, [655, 2], [9, 11], 1)?,
            ground: Ground::new(ctx, &sprite_image, [2, 54], [30, 12])?,
            dino: DinoData::new(ctx, &sprite_image, [848, 2], [44, 47], [1112, 19], [59, 30])?,
        })
    }
}
