mod digits;
mod dino;
mod ground;
pub mod tile;

use crate::game::assets::sprites::digits::Digits;
use crate::game::assets::sprites::dino::DinoData;
use crate::game::assets::sprites::ground::Ground;
use ggez::error::GameResult;
use ggez::graphics::{self, FilterMode};
use ggez::Context;

pub struct SpriteData {
    pub sprite_sheet: graphics::Image,
    pub digits: Digits,
    pub ground: Ground,
    pub dino: DinoData,
}

impl SpriteData {
    pub fn new(ctx: &mut Context, _scale: usize) -> GameResult<Self> {
        let mut sprite_sheet = graphics::Image::new(ctx, "/sprites.png")?;
        sprite_sheet.set_filter(FilterMode::Nearest);

        let sprite_sheet_size = [sprite_sheet.width() as f32, sprite_sheet.height() as f32];

        Ok(SpriteData {
            sprite_sheet,
            digits: Digits::new(sprite_sheet_size, [655.0, 2.0], [9.0, 11.0], 1.0),
            ground: Ground::new(sprite_sheet_size, [2.0, 54.0], [30.0, 12.0])?,
            dino: DinoData::new(
                sprite_sheet_size,
                [848.0, 2.0],
                [44.0, 47.0],
                [1112.0, 19.0],
                [59.0, 30.0],
            ),
        })
    }
}
