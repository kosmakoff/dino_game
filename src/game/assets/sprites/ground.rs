use crate::game::assets::sprites::tile::Tile;
use ggez::GameResult;
use std::mem::{self, MaybeUninit};

// 2x54 1200x12 ; single tile W=30 H=12

pub struct Ground([Tile; 40]);

impl Ground {
    pub fn new(
        sprite_sheet_size: [usize; 2],
        position: [usize; 2],
        size: [usize; 2],
    ) -> GameResult<Self> {
        let [sprite_sheet_width, sprite_sheet_height] = sprite_sheet_size;
        let [start_x, start_y] = position;
        let [width, height] = size;

        let mut ground_data: [MaybeUninit<Tile>; 40] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for index in 0..40 {
            let position = [
                (start_x + width * index) as f32 / sprite_sheet_width as f32,
                start_y as f32 / sprite_sheet_height as f32,
            ];
            ground_data[index] = MaybeUninit::new(Tile::new(
                position,
                [
                    width as f32 / sprite_sheet_width as f32,
                    height as f32 / sprite_sheet_height as f32,
                ],
            ));
        }

        let ground_data = unsafe { mem::transmute::<_, [Tile; 40]>(ground_data) };

        Ok(Ground(ground_data))
    }

    pub fn get_tile(&self, index: usize) -> &Tile {
        let Ground(ground_data) = self;
        &ground_data[index]
    }
}
