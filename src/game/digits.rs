use image::{DynamicImage, GenericImageView};
use opengl_graphics::{Texture, TextureSettings};

pub type Digits = [Texture; 10];

/// Gets single texture from sprite sheet.
fn get_texture(
    sprite_sheet: &DynamicImage,
    texture_settings: &TextureSettings,
    position: [usize; 2],
    size: [usize; 2],
) -> Texture {
    let [x, y] = position;
    let [width, height] = size;
    let one_character_image = sprite_sheet
        .view(x as u32, y as u32, width as u32, height as u32)
        .to_image();
    let one_character_texture = Texture::from_image(&one_character_image, texture_settings);
    one_character_texture
}

/// Loads digits' textures from sprite sheet.
pub fn load_digits(
    sprite_sheet: &DynamicImage,
    position: [usize; 2],
    size: [usize; 2],
    padding: usize,
) -> Digits {
    let texture_settings = &TextureSettings::new();

    let [width, _] = size;
    let [x, y] = position;

    [
        get_texture(sprite_sheet, texture_settings, [x, y], size),
        get_texture(sprite_sheet, texture_settings, [x + (width + 1), y], size),
        get_texture(
            sprite_sheet,
            texture_settings,
            [x + (width + 1) * 2, y],
            size,
        ),
        get_texture(
            sprite_sheet,
            texture_settings,
            [x + (width + 1) * 3, y],
            size,
        ),
        get_texture(
            sprite_sheet,
            texture_settings,
            [x + (width + 1) * 4, y],
            size,
        ),
        get_texture(
            sprite_sheet,
            texture_settings,
            [x + (width + 1) * 5, y],
            size,
        ),
        get_texture(
            sprite_sheet,
            texture_settings,
            [x + (width + 1) * 6, y],
            size,
        ),
        get_texture(
            sprite_sheet,
            texture_settings,
            [x + (width + 1) * 7, y],
            size,
        ),
        get_texture(
            sprite_sheet,
            texture_settings,
            [x + (width + 1) * 8, y],
            size,
        ),
        get_texture(
            sprite_sheet,
            texture_settings,
            [x + (width + 1) * 9, y],
            size,
        ),
    ]
}
