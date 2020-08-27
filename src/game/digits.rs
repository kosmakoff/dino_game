use ggez::graphics::Image;
use ggez::{Context, GameResult};
use image::{DynamicImage, GenericImageView};

pub type Digits = [Image; 10];

/// Gets single image from sprite sheet.
fn get_image(
    ctx: &mut Context,
    sprite_sheet: &DynamicImage,
    position: [usize; 2],
    size: [usize; 2],
) -> GameResult<Image> {
    let [x, y] = position;
    let [width, height] = size;
    let one_character_image = sprite_sheet
        .view(x as u32, y as u32, width as u32, height as u32)
        .to_image()
        .to_vec();

    Image::from_rgba8(ctx, width as u16, height as u16, &one_character_image)
}

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

    Ok([
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
    ])
}
