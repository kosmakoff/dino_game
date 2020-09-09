use crate::game::helpers::get_image;
use ggez::graphics::Image;
use ggez::{Context, GameResult};
use image::DynamicImage;

pub struct Digits([Image; 10]);

impl Digits {
    pub fn new(
        ctx: &mut Context,
        sprite_sheet: &DynamicImage,
        position: [usize; 2],
        size: [usize; 2],
        padding: usize,
    ) -> GameResult<Self> {
        let [width, _] = size;
        let [x, y] = position;

        let images = [
            get_image(ctx, sprite_sheet, [x, y], size)?,
            get_image(ctx, sprite_sheet, [x + (width + padding), y], size)?,
            get_image(ctx, sprite_sheet, [x + (width + padding) * 2, y], size)?,
            get_image(ctx, sprite_sheet, [x + (width + padding) * 3, y], size)?,
            get_image(ctx, sprite_sheet, [x + (width + padding) * 4, y], size)?,
            get_image(ctx, sprite_sheet, [x + (width + padding) * 5, y], size)?,
            get_image(ctx, sprite_sheet, [x + (width + padding) * 6, y], size)?,
            get_image(ctx, sprite_sheet, [x + (width + padding) * 7, y], size)?,
            get_image(ctx, sprite_sheet, [x + (width + padding) * 8, y], size)?,
            get_image(ctx, sprite_sheet, [x + (width + padding) * 9, y], size)?,
        ];

        Ok(Digits(images))
    }
}
