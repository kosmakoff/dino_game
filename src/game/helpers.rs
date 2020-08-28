use ggez::error::GameResult;
use ggez::graphics::Image;
use ggez::Context;
use image::{DynamicImage, GenericImageView};

/// Gets single image from sprite sheet.
pub fn get_image(
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
