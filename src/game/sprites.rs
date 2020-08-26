use crate::game::digits::{load_digits, Digits};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};

pub struct SpriteData {
    pub digits: Digits,
}

impl SpriteData {
    pub fn new(scale: usize) -> Self {
        let sprite_data = include_bytes!("../../assets/sprites.png");

        let sprite_image =
            image::load_from_memory_with_format(sprite_data, image::ImageFormat::Png)
                .expect("Failed to load sprites");

        SpriteData {
            digits: load_digits(&sprite_image, [655, 2], [9, 11], 1),
        }
    }
}
