use crate::game::assets::sprites::tile::Tile;

pub struct Digits([Tile; 10]);

impl Digits {
    pub fn new(
        sprite_sheet_size: [usize; 2],
        position: [usize; 2],
        size: [usize; 2],
        padding: usize,
    ) -> Self {
        let [sprite_sheet_width, sprite_sheet_height] = sprite_sheet_size;
        let [width, height] = size;
        let [x, y] = position;
        let size = [width as f32, height as f32];

        let width_and_padding = (width + padding) as f32;

        let digits = [
            Tile::new(
                [
                    x as f32 / sprite_sheet_width as f32,
                    y as f32 / sprite_sheet_height as f32,
                ],
                size,
            ),
            Tile::new(
                [
                    (x as f32 + width_and_padding) / sprite_sheet_width as f32,
                    y as f32,
                ],
                size,
            ),
            Tile::new(
                [
                    (x as f32 + width_and_padding * 2.0) / sprite_sheet_width as f32,
                    y as f32,
                ],
                size,
            ),
            Tile::new(
                [
                    (x as f32 + width_and_padding * 3.0) / sprite_sheet_width as f32,
                    y as f32,
                ],
                size,
            ),
            Tile::new(
                [
                    (x as f32 + width_and_padding * 4.0) / sprite_sheet_width as f32,
                    y as f32,
                ],
                size,
            ),
            Tile::new(
                [
                    (x as f32 + width_and_padding * 5.0) / sprite_sheet_width as f32,
                    y as f32,
                ],
                size,
            ),
            Tile::new(
                [
                    (x as f32 + width_and_padding * 6.0) / sprite_sheet_width as f32,
                    y as f32,
                ],
                size,
            ),
            Tile::new(
                [
                    (x as f32 + width_and_padding * 7.0) / sprite_sheet_width as f32,
                    y as f32,
                ],
                size,
            ),
            Tile::new(
                [
                    (x as f32 + width_and_padding * 8.0) / sprite_sheet_width as f32,
                    y as f32,
                ],
                size,
            ),
            Tile::new(
                [
                    (x as f32 + width_and_padding * 9.0) / sprite_sheet_width as f32,
                    y as f32,
                ],
                size,
            ),
        ];

        Digits(digits)
    }
}
