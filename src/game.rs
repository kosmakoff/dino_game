mod assets;
mod objects;

use crate::game::assets::sounds::Sound;
use crate::game::assets::sprites::tile::Tile;
use crate::game::assets::Assets;
use crate::game::objects::dino::Dino;
use ggez::audio::SoundSource;
use ggez::event::quit;
use ggez::event::EventHandler;
use ggez::graphics::set_window_title;
use ggez::graphics::DrawParam;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::{graphics, timer, Context, GameResult};
use oorandom::Rand32;

const MAX_JUMP_HEIGHT: f32 = 101.0;
const JUMP_DURATION: f32 = 0.28;

const JUMP_VELOCITY: f32 = 2.0 * MAX_JUMP_HEIGHT / JUMP_DURATION;
const GRAVITY: f32 = JUMP_VELOCITY / JUMP_DURATION;

pub struct Game {
    assets: Assets,
    dino: Dino,
    rng: Rand32,
    step: bool,
    position_vertical: f32,
    speed_vertical: f32,
    screen_width: f32,
    screen_height: f32,
    screen_scale: [f32; 2],
}

pub struct DrawContext {
    pub screen_size: [f32; 2],
    pub screen_scale: [f32; 2],
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let assets = Assets::new(ctx)?;
        let mut seed: [u8; 8] = [0; 8];
        getrandom::getrandom(&mut seed[..]).expect("Could not create RNG seed");
        let rng = Rand32::new(u64::from_ne_bytes(seed));
        let (width, height) = graphics::drawable_size(ctx);
        Ok(Game {
            assets: assets,
            dino: Dino::new(),
            rng,
            step: false,
            position_vertical: 0.0,
            speed_vertical: 0.0,
            screen_width: width,
            screen_height: height,
            screen_scale: [2.0, 2.0],
        })
    }

    fn get_draw_context(&self) -> DrawContext {
        DrawContext {
            screen_size: [self.screen_width, self.screen_height],
            screen_scale: self.screen_scale,
        }
    }

    fn play_sound(&mut self, ctx: &Context, sound: Sound) {
        let sound_to_play = match sound {
            Sound::ButtonPress => &mut self.assets.sounds.button_press,
            Sound::Hit => &mut self.assets.sounds.hit,
            Sound::ScoreReached => &mut self.assets.sounds.score_reached,
        };

        let _ = sound_to_play.play(ctx);
    }
}

fn make_draw_param(tile: &Tile, scale: [f32; 2], dest: [f32; 2]) -> DrawParam {
    let [scale_x, scale_y] = scale;
    let [dest_x, dest_y] = dest;

    DrawParam::default()
        .src(tile.into())
        .scale(scale)
        .dest([dest_x * scale_x, dest_y * scale_y])
}

impl EventHandler<ggez::GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        set_window_title(ctx, &format!("Dino Game - {:.1} FPS", fps));

        self.dino.update(ctx);

        // // set speed
        // self.speed_vertical += dt_float * GRAVITY;
        // self.position_vertical += dt_float * self.speed_vertical;
        // if self.position_vertical > 0.0 {
        //     self.position_vertical = 0.0;
        //     self.speed_vertical = 0.0;
        // }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        const BG_DAY: [f32; 4] = [247.0 / 255.0, 247.0 / 255.0, 247.0 / 255.0, 1.0];
        // const BG_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let draw_context = self.get_draw_context();

        graphics::clear(ctx, BG_DAY.into());

        // draw ground
        graphics::draw(
            ctx,
            &self.assets.sprites.sprite_sheet,
            make_draw_param(
                self.assets.sprites.ground.get_tile(24),
                self.screen_scale,
                [100.0, 283.0],
            ),
        )?;

        graphics::draw(
            ctx,
            &self.assets.sprites.sprite_sheet,
            make_draw_param(
                self.assets.sprites.ground.get_tile(25),
                self.screen_scale,
                [130.0, 283.0],
            ),
        )?;

        graphics::draw(
            ctx,
            &self.assets.sprites.sprite_sheet,
            make_draw_param(
                self.assets.sprites.ground.get_tile(2),
                self.screen_scale,
                [160.0, 283.0],
            ),
        )?;

        graphics::draw(
            ctx,
            &self.assets.sprites.sprite_sheet,
            make_draw_param(
                self.assets.sprites.ground.get_tile(35),
                self.screen_scale,
                [190.0, 283.0],
            ),
        )?;

        graphics::draw(
            ctx,
            &self.assets.sprites.sprite_sheet,
            make_draw_param(
                self.assets.sprites.ground.get_tile(36),
                self.screen_scale,
                [220.0, 283.0],
            ),
        )?;

        // draw dino
        self.dino.draw(ctx, &self.assets, &draw_context)?;

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
            KeyCode::Space => {
                // start jump
                self.play_sound(ctx, Sound::ButtonPress);
                self.speed_vertical = -JUMP_VELOCITY;
            }
            // KeyCode::Return => self.play_sound(Sound::ButtonPress),
            // KeyCode::RShift => self.play_sound(Sound::ScoreReached),
            KeyCode::Escape => quit(ctx),
            _ => (),
        };
    }
}
