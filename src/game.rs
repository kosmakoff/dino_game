extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

mod digits;
mod sprites;

use crate::game::sprites::SpriteData;
use crate::piston::AdvancedWindow;
use glutin_window::GlutinWindow as Window;
use graphics::Graphics;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rand::prelude::*;

pub struct Game {
    graphics: GlGraphics,
    sprites: SpriteData,
    rng: ThreadRng,
    pub sound_device: rodio::Device,
}

impl Game {
    pub fn new(gl: GlGraphics) -> Self {
        Game {
            graphics: gl,
            sprites: SpriteData::new(1),
            rng: rand::thread_rng(),
            sound_device: rodio::default_output_device().unwrap(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const BG_DAY: [f32; 4] = [247.0 / 255.0, 247.0 / 255.0, 247.0 / 255.0, 1.0];

        let sprites = &self.sprites;
        let rng = &mut self.rng;

        self.graphics.draw(args.viewport(), |c, gl| {
            clear(BG_DAY, gl);

            let transform = c.transform.trans_pos([100.0, 100.0]);

            let image = Image::new();

            let random_index = (rng.gen::<f64>() * 10.0) as usize;
            image.draw(&sprites.digits[random_index], &c.draw_state, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {}
}
