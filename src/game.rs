mod assets;
mod objects;

use crate::game::assets::sounds::Sound;
use crate::game::assets::sprites::tile::Tile;
use crate::game::assets::Assets;
use crate::game::objects::dino::Dino;
use crate::game::objects::ground::Ground;
use ggez::audio::SoundSource;
use ggez::event::quit;
use ggez::event::EventHandler;
use ggez::graphics::set_window_title;
use ggez::graphics::DrawParam;
use ggez::input::keyboard::{KeyCode, KeyMods};
use ggez::GameError;
use ggez::{graphics, timer, Context, GameResult};
use oorandom::Rand32;

const MAX_JUMP_HEIGHT: f32 = 101.0;
const JUMP_DURATION: f32 = 0.28;

const JUMP_VELOCITY: f32 = 2.0 * MAX_JUMP_HEIGHT / JUMP_DURATION;
const GRAVITY: f32 = JUMP_VELOCITY / JUMP_DURATION;

const GROUND_LEVEL: f32 = 300.0;

pub struct Game {
    assets: Assets,
    dino: Dino,
    ground: Ground,
    rng: Rand32,
    screen_width: f32,
    screen_height: f32,
    screen_scale: [f32; 2],
}

pub struct DrawContext {
    pub screen_size: [f32; 2],
    pub screen_scale: [f32; 2],
}

fn create_rng() -> GameResult<Rand32> {
    let mut seed: [u8; 8] = [0; 8];
    match getrandom::getrandom(&mut seed[..]) {
        Ok(_) => (),
        Err(error) => {
            return Err(GameError::CustomError(format!(
                "Failed to initialize the RNG: {}",
                error
            )))
        }
    };
    let rng = Rand32::new(u64::from_ne_bytes(seed));
    Ok(rng)
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let assets = Assets::new(ctx)?;
        let rng = create_rng()?;
        let (width, height) = graphics::drawable_size(ctx);

        Ok(Game {
            assets: assets,
            dino: Dino::new(),
            ground: Ground::new(),
            rng,
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
        .offset([0.0, 1.0])
}

impl EventHandler<ggez::GameError> for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let fps = timer::fps(ctx);
        set_window_title(ctx, &format!("Dino Game - {:.1} FPS", fps));

        self.dino.update(ctx);

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        const BG_DAY: [f32; 4] = [247.0 / 255.0, 247.0 / 255.0, 247.0 / 255.0, 1.0];
        // const BG_BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

        let draw_context = self.get_draw_context();

        graphics::clear(ctx, BG_DAY.into());

        self.ground.draw(ctx, &self.assets, &draw_context)?;
        self.dino.draw(ctx, &self.assets, &draw_context)?;

        graphics::present(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        repeat: bool,
    ) {
        self.dino.key_down_event(ctx, keycode, keymods, repeat);

        match keycode {
            KeyCode::Space => {
                // start jump
                self.play_sound(ctx, Sound::ButtonPress);
            }
            // KeyCode::Return => self.play_sound(Sound::ButtonPress),
            // KeyCode::RShift => self.play_sound(Sound::ScoreReached),
            KeyCode::Escape => quit(ctx),
            _ => (),
        };
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: KeyMods) {
        self.dino.key_up_event(ctx, keycode, keymods);
    }
}
