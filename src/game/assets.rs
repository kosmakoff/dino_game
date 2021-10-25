pub mod sounds;
pub mod sprites;

use crate::game::assets::sounds::Sounds;
use crate::game::assets::sprites::SpriteData;
use ggez::{Context, GameResult};

pub struct Assets {
    pub sprites: SpriteData,
    pub sounds: Sounds,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Assets {
            sprites: SpriteData::new(ctx, 1)?,
            sounds: Sounds::new(ctx)?,
        })
    }
}
