use std::mem::{self, MaybeUninit};

use crate::game::assets::sprites::tile::Tile;

pub struct Digits([Tile; 10]);

impl Digits {
    pub fn new(
        sprite_sheet_size: [f32; 2],
        position: [f32; 2],
        size: [f32; 2],
        padding: f32,
    ) -> Self {
        let [width, height] = size;
        let [x, y] = position;
        let size = [width as f32, height as f32];

        let width_and_padding = (width + padding) as f32;

        let mut digits_data: [MaybeUninit<Tile>; 10] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for index in 0..10 {
            let position = [x + width_and_padding * index as f32, y];
            digits_data[index] = MaybeUninit::new(Tile::new(position, size, sprite_sheet_size));
        }

        let digits_data = unsafe { mem::transmute::<_, [Tile; 10]>(digits_data) };

        Digits(digits_data)
    }
}
