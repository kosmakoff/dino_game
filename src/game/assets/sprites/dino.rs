use crate::game::assets::sprites::tile::Tile;

pub struct DinoData {
    pub idle: Tile,
    pub blink: Tile,
    pub walk1: Tile,
    pub walk2: Tile,
    pub crouched1: Tile,
    pub crouched2: Tile,
    pub hit: Tile,
}

impl DinoData {
    pub fn new(
        sprite_sheet_size: [usize; 2],
        normal_state_start: [usize; 2],
        normal_state_size: [usize; 2],
        crouched_state_start: [usize; 2],
        crouched_state_size: [usize; 2],
    ) -> Self {
        let [sprite_sheet_width, sprite_sheet_height] = sprite_sheet_size;
        let [sprite_sheet_width, sprite_sheet_height] =
            [sprite_sheet_width as f32, sprite_sheet_height as f32];

        // 848,2 44,47 - first block

        let [normal_state_start_x, normal_state_start_y] = normal_state_start;
        let [normal_state_size_width, normal_state_size_height] = normal_state_size;
        let normal_state_size = [
            normal_state_size_width as f32 / sprite_sheet_width,
            normal_state_size_height as f32 / sprite_sheet_height,
        ];

        let image_idle = Tile::new(
            [
                normal_state_start_x as f32 / sprite_sheet_width,
                normal_state_start_y as f32 / sprite_sheet_height,
            ],
            normal_state_size,
        );

        let image_blink = Tile::new(
            [
                (normal_state_start_x + normal_state_size_width) as f32 / sprite_sheet_width,
                normal_state_start_y as f32 / sprite_sheet_height,
            ],
            normal_state_size,
        );

        let image_walk1 = Tile::new(
            [
                (normal_state_start_x + normal_state_size_width * 2) as f32 / sprite_sheet_width,
                normal_state_start_y as f32 / sprite_sheet_height,
            ],
            normal_state_size,
        );

        let image_walk2 = Tile::new(
            [
                (normal_state_start_x + normal_state_size_width * 3) as f32 / sprite_sheet_width,
                normal_state_start_y as f32 / sprite_sheet_height,
            ],
            normal_state_size,
        );

        let image_hit = Tile::new(
            [
                (normal_state_start_x + normal_state_size_width * 3) as f32 / sprite_sheet_width,
                normal_state_start_y as f32 / sprite_sheet_height,
            ],
            normal_state_size,
        );

        let [crouched_state_start_x, crouched_state_start_y] = crouched_state_start;
        let [crouched_state_size_width, crouched_state_size_height] = crouched_state_size;
        let crouched_state_size = [
            crouched_state_size_width as f32 / sprite_sheet_width,
            crouched_state_size_height as f32 / sprite_sheet_height,
        ];

        let image_crouched1 = Tile::new(
            [
                crouched_state_start_x as f32 / sprite_sheet_width,
                crouched_state_start_y as f32 / sprite_sheet_height,
            ],
            crouched_state_size,
        );

        let image_crouched2 = Tile::new(
            [
                (crouched_state_start_x + crouched_state_size_width) as f32 / sprite_sheet_width,
                crouched_state_start_y as f32 / sprite_sheet_height,
            ],
            crouched_state_size,
        );

        DinoData {
            idle: image_idle,
            blink: image_blink,
            walk1: image_walk1,
            walk2: image_walk2,
            crouched1: image_crouched1,
            crouched2: image_crouched2,
            hit: image_hit,
        }
    }
}
