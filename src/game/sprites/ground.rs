use crate::game::helpers::get_image;
use ggez::graphics::Image;
use ggez::{Context, GameResult};
use image::DynamicImage;
use std::mem::{self, MaybeUninit};

// 2x54 1200x12 ; single tile W=30 H=12
pub struct Ground([Image; 40]);

impl Ground {
    pub fn new(
        ctx: &mut Context,
        sprite_sheet: &DynamicImage,
        position: [usize; 2],
        size: [usize; 2],
    ) -> GameResult<Self> {
        let [start_x, start_y] = position;
        let [width, _] = size;

        let mut ground_data: [MaybeUninit<Image>; 40] =
            unsafe { MaybeUninit::uninit().assume_init() };

        for index in 0..40 {
            let position = [start_x + width * index, start_y];
            let image = get_image(ctx, sprite_sheet, position, size)?;
            ground_data[index] = MaybeUninit::new(image);
        }

        let ground_data = unsafe { mem::transmute::<_, [Image; 40]>(ground_data) };

        Ok(Ground(ground_data))
    }

    pub fn get_tile(&self, index: usize) -> &Image {
        let Ground(ground_data) = self;
        &ground_data[index]
    }
}
