use crate::game::helpers::get_image;
use ggez::graphics::{FilterMode, Image};
use ggez::{Context, GameResult};
use image::DynamicImage;

pub type Digits = [Image; 10];

/// Loads digits' image from sprite sheet.
pub fn load_digits(
    ctx: &mut Context,
    sprite_sheet: &DynamicImage,
    position: [usize; 2],
    size: [usize; 2],
    padding: usize,
) -> GameResult<Digits> {
    let [width, _] = size;
    let [x, y] = position;

    let mut images = [
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

    for image in &mut images {
        image.set_filter(FilterMode::Nearest);
    }

    Ok(images)
}
