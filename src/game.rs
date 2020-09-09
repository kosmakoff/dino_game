mod helpers;
mod sounds;
mod sprites;

use crate::game::sounds::Sound;
use crate::game::sounds::Sounds;
use crate::game::sprites::SpriteData;
use ggez::event::quit;
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
    step: bool,
    timer: u128,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Game {
            sprites: SpriteData::new(ctx, 1)?,
            rng: rand::thread_rng(),
            sound_device: rodio::default_output_device().unwrap(),
            sounds: Sounds::new(),
            step: false,
            timer: 0,
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

        let dt = timer::delta(ctx);
        let dt_micros = dt.as_micros();
        self.timer += dt_micros;

        if self.timer >= 100_000 {
            self.timer -= 100_000;
            self.step = !self.step;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        const BG_DAY: [f32; 4] = [247.0 / 255.0, 247.0 / 255.0, 247.0 / 255.0, 1.0];
        // const BG_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        graphics::clear(ctx, BG_DAY.into());

        // 24,25
        // 35,36

        graphics::draw(
            ctx,
            self.sprites.ground.get_tile(24),
            (na::Point2::new(100.0, 283.0),),
        )?;

        graphics::draw(
            ctx,
            self.sprites.ground.get_tile(25),
            (na::Point2::new(130.0, 283.0),),
        )?;

        graphics::draw(
            ctx,
            self.sprites.ground.get_tile(2),
            (na::Point2::new(160.0, 283.0),),
        )?;

        graphics::draw(
            ctx,
            self.sprites.ground.get_tile(35),
            (na::Point2::new(190.0, 283.0),),
        )?;

        graphics::draw(
            ctx,
            self.sprites.ground.get_tile(36),
            (na::Point2::new(220.0, 283.0),),
        )?;

        let image = match self.step {
            true => &self.sprites.dino.walk1,
            false => &self.sprites.dino.walk2,
        };

        graphics::draw(ctx, image, (na::Point2::new(100.0, 250.0),))?;

        /*
        let mesh_builder = &mut graphics::MeshBuilder::new();

        for x in 0..=640 {
            if x % 20 != 0 {
                continue;
            }

            mesh_builder.line(
                &[
                    na::Point2::new(x as f32, 0.0),
                    na::Point2::new(x as f32, 480.0),
                ],
                3.0,
                BG_BLACK.into(),
            )?;
        }

        for y in 0..=480 {
            if y % 20 != 0 {
                continue;
            }

            mesh_builder.line(
                &[
                    na::Point2::new(0.0, y as f32),
                    na::Point2::new(640.0, y as f32),
                ],
                3.0,
                BG_BLACK.into(),
            )?;
        }

        let mesh = mesh_builder.build(ctx)?;
        graphics::draw(ctx, &mesh, (na::Point2::origin(),))?;
        */

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space => self.play_sound(Sound::Hit),
            KeyCode::Return => self.play_sound(Sound::ButtonPress),
            KeyCode::RShift => self.play_sound(Sound::ScoreReached),
            KeyCode::Escape => quit(ctx),
            _ => (),
        };
    }
}
