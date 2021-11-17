use std::collections::VecDeque;

use ggez::{graphics, Context, GameResult};
use oorandom::Rand32;

use crate::game::{
    assets::{sprites::tile::Tile, Assets},
    make_draw_param, DrawContext, GROUND_LEVEL,
};

const BACK_TILES_DISTANCE: f32 = 100.0;
const MAX_TOTAL_TILES_WIDTH: f32 = 450.0;

pub struct Ground {
    past_offset: f32,
    tiles_total_width: f32,
    tiles: VecDeque<Tile>,
}

impl Ground {
    pub fn new(assets: &Assets) -> Self {
        let tiles = VecDeque::from([
            assets.sprites.ground.get_tile(24),
            assets.sprites.ground.get_tile(25),
            assets.sprites.ground.get_tile(2),
            assets.sprites.ground.get_tile(35),
            assets.sprites.ground.get_tile(36),
        ]);

        let tiles_total_width = tiles
            .iter()
            .map(|t| {
                let [w, _] = t.size_raw();
                w
            })
            .sum();

        Self {
            past_offset: 0.0,
            tiles_total_width,
            tiles,
        }
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        draw_context: &DrawContext,
    ) -> GameResult<()> {
        let mut current_total_width: f32 = 0.0;
        for tile in self.tiles.iter() {
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

    pub(crate) fn expand(&mut self, rng: &mut Rand32, assets: &Assets) {
        while self.tiles_total_width - self.past_offset < MAX_TOTAL_TILES_WIDTH {
            let mut new_tiles_width = 0.0;

            let new_tile_index = rng.rand_range(0..40);
            let new_tile_index = match new_tile_index {
                25 => 24,
                36 => 35,
                _ => new_tile_index,
            };
            let new_tile = assets.sprites.ground.get_tile(new_tile_index as usize);
            let [tile_width, _] = new_tile.size_raw();
            new_tiles_width += tile_width;
            self.tiles.push_back(new_tile);

            if new_tile_index == 24 || new_tile_index == 35 {
                let additional_tile_index = new_tile_index + 1;
                let additional_tile = assets
                    .sprites
                    .ground
                    .get_tile(additional_tile_index as usize);
                let [tile_width, _] = additional_tile.size_raw();
                new_tiles_width += tile_width;
                self.tiles.push_back(additional_tile);
            }

            self.tiles_total_width += new_tiles_width;
        }
    }

    pub(crate) fn advance(&mut self, distance: f32) {
        self.past_offset += distance;

        loop {
            let tile = match self.tiles.front() {
                Some(tile) => tile,
                None => break,
            };

            let [tile_width, _] = tile.size_raw();
            if self.past_offset - BACK_TILES_DISTANCE < tile_width {
                break;
            }

            self.past_offset -= tile_width;
            self.tiles_total_width -= tile_width;
            self.tiles.pop_front();
        }
    }
}
