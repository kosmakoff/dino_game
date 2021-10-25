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

        let sprite_sheet_size = [
            sprite_sheet.width() as usize,
            sprite_sheet.height() as usize,
        ];

        Ok(SpriteData {
            sprite_sheet,
            digits: Digits::new(sprite_sheet_size, [655, 2], [9, 11], 1),
            ground: Ground::new(sprite_sheet_size, [2, 54], [30, 12])?,
            dino: DinoData::new(sprite_sheet_size, [848, 2], [44, 47], [1112, 19], [59, 30]),
        })
    }
}
