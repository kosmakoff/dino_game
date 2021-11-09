use ggez::{graphics, Context, GameResult};

use crate::game::{
    assets::{sprites::tile::Tile, Assets},
    make_draw_param, DrawContext, GROUND_LEVEL,
};

const MAX_TOTAL_TILES_WIDTH: f32 = 400.0;

pub struct Ground {
    past_offset: f32,
    tiles_total_width: f32,
    tiles: Vec<usize>,
}

impl Ground {
    pub fn new() -> Self {
        Self {
            past_offset: 0.0,
            tiles_total_width: 0.0,
            tiles: vec![24, 25, 2, 35, 36],
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        draw_context: &DrawContext,
    ) -> GameResult<()> {
        let mut current_total_width: f32 = 0.0;
        for tile_index in &self.tiles {
            let tile = assets.sprites.ground.get_tile(*tile_index);
            graphics::draw(
                ctx,
                &assets.sprites.sprite_sheet,
                make_draw_param(
                    tile,
                    draw_context.screen_scale,
                    [100.0 - self.past_offset + current_total_width, GROUND_LEVEL],
                ),
            )?;

            let [tile_width, _] = tile.size_raw();
            current_total_width += tile_width;
        }

        Ok(())
    }

    pub fn expand() {}
}
