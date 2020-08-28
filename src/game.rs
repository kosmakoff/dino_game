mod digits;
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
    _rng: ThreadRng,
    sound_device: rodio::Device,
    sounds: Sounds,
    current_index: usize,
    position: na::Point2<f32>,
    velocity: na::Vector2<f32>,
}

fn clamp_position_speed(
    max_size: &[f32; 2],
    position: &mut na::Point2<f32>,
    velocity: &mut na::Vector2<f32>,
) -> bool {
    let [max_width, max_height] = max_size;

    let mut changed_outer = false;

    loop {
        let mut changed = false;
        if position.x < 0.0 {
            position.x = -position.x;
            velocity.x = -velocity.x;
            changed = true;
        }

        if position.y < 0.0 {
            position.y = -position.y;
            velocity.y = -velocity.y;
            changed = true;
        }

        if position.x >= *max_width {
            position.x = *max_width * 2.0 - position.x;
            velocity.x = -velocity.x;
            changed = true;
        }

        if position.y >= *max_height {
            position.y = *max_height * 2.0 - position.y;
            velocity.y = -velocity.y;
            changed = true;
        }

        if changed {
            changed_outer = true;
        } else {
            break;
        }
    }

    changed_outer
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Game {
            sprites: SpriteData::new(ctx, 1)?,
            _rng: rand::thread_rng(),
            sound_device: rodio::default_output_device().unwrap(),
            sounds: Sounds::new(),
            current_index: 0,
            position: na::Point2::new(0.0, 0.0),
            velocity: na::Vector2::new(64.0, 64.0),
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
        let shift = self.velocity.scale(dt.as_micros() as f32 / 1000000.0);

        self.position = self.position + shift;
        if clamp_position_speed(
            &[
                640.0 - self.sprites.dino.width() as f32,
                480.0 - self.sprites.dino.height() as f32,
            ],
            &mut self.position,
            &mut self.velocity,
        ) {
            self.play_sound(Sound::Hit);
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        const BG_DAY: [f32; 4] = [247.0 / 255.0, 247.0 / 255.0, 247.0 / 255.0, 1.0];
        // const BG_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        graphics::clear(ctx, BG_DAY.into());
        graphics::draw(
            ctx,
            //&self.sprites.digits[self.current_index],
            &self.sprites.dino,
            (self.position,),
        )?;

        /*
        graphics::draw(
            ctx,
            &self.sprites.dino,
            (na::Point2::new(
                320.0 - self.sprites.dino.width() as f32 / 2.0,
                240.0 - self.sprites.dino.height() as f32 / 2.0,
            ),),
        )?;
        */

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
