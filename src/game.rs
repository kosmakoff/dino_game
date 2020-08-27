mod digits;
mod sounds;
mod sprites;

use crate::game::sounds::Sound;
use crate::game::sounds::Sounds;
use crate::game::sprites::SpriteData;
use ggez::event::EventHandler;
use ggez::graphics::set_window_title;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::nalgebra as na;
use ggez::{graphics, timer, Context, GameResult};
use rand::prelude::*;
use rodio::Source;

pub struct Game {
    sprites: SpriteData,
    rng: ThreadRng,
    sound_device: rodio::Device,
    sounds: Sounds,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Game {
            sprites: SpriteData::new(ctx, 1)?,
            rng: rand::thread_rng(),
            sound_device: rodio::default_output_device().unwrap(),
            sounds: Sounds::new(),
        })
    }

    fn play_sound(&self, sound: Sound) {
        let sound_to_play = match sound {
            Sound::ButtonPress => &self.sounds.button_press,
            Sound::Hit => &self.sounds.hit,
            Sound::ScoreReached => &self.sounds.score_reached,
        };

        let cursor_clone = sound_to_play.clone();
        let source = rodio::Decoder::new(cursor_clone).unwrap().convert_samples();
        rodio::play_raw(&self.sound_device, source);
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        set_window_title(ctx, &format!("Dino Game - {:.1} FPS", fps));
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        const BG_DAY: [f32; 4] = [247.0 / 255.0, 247.0 / 255.0, 247.0 / 255.0, 1.0];

        graphics::clear(ctx, BG_DAY.into());
        let random_index = (self.rng.gen::<f64>() * 10.0) as usize;
        let position = na::Point2::new(100.0, 100.0);
        graphics::draw(ctx, &self.sprites.digits[random_index], (position,))?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space => self.play_sound(Sound::Hit),
            KeyCode::Return => self.play_sound(Sound::ButtonPress),
            KeyCode::RShift => self.play_sound(Sound::ScoreReached),
            _ => (),
        };
    }
}
