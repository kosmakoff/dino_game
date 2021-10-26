use crate::game::make_draw_param;
use crate::game::Assets;
use crate::game::DrawContext;
use ggez::graphics;
use ggez::timer;
use ggez::Context;
use ggez::GameResult;

const BLINKING_DURATION: u128 = 100_000;
const BLINKING_INTERVAL: u128 = 2_500_000;

const WALKING_STEP_DURATION: u128 = 100_000;

pub struct IdleState {
    since: u128,
    pub is_blinking: bool,
}

pub struct WalkingState {
    since: u128,
    pub is_left_leg: bool,
}

pub struct CrouchedState {}

pub enum DinoState {
    Idle(IdleState),
    Walking(WalkingState),
    Jumping,
    Crouched(CrouchedState),
    Hit,
}

impl DinoState {
    pub fn new() -> Self {
        DinoState::Idle(IdleState {
            since: 0,
            is_blinking: false,
        })
    }
}

pub struct Dino(DinoState);

impl Dino {
    pub fn new() -> Self {
        Dino(DinoState::new())
    }
    pub fn update(&mut self, ctx: &mut Context) {
        let dt = timer::delta(ctx);
        let dt_micros = dt.as_micros();
        let _dt_float = dt.as_secs_f32();

        let Dino(ref mut dino_state) = self;

        match dino_state {
            DinoState::Idle(ref mut idle) => {
                idle.since += dt_micros;

                if idle.since >= BLINKING_INTERVAL {
                    idle.since -= BLINKING_INTERVAL;
                }

                idle.is_blinking = idle.since < BLINKING_DURATION;
            }

            DinoState::Walking(ref mut walking) => {
                walking.since += dt_micros;

                if walking.since >= WALKING_STEP_DURATION {
                    walking.since -= WALKING_STEP_DURATION;
                    walking.is_left_leg = !walking.is_left_leg;
                }
            }
            _ => (),
        }
    }
    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &Assets,
        draw_context: &DrawContext,
    ) -> GameResult<()> {
        let Dino(ref dino_state) = self;

        match dino_state {
            DinoState::Idle(ref idle) => self.draw_idle(idle, ctx, assets, draw_context)?,
            DinoState::Walking(ref walking) => {
                self.draw_walking(walking, ctx, assets, draw_context)?
            }
            _ => (),
        };

        Ok(())
    }

    fn draw_idle(
        &self,
        idle_state: &IdleState,
        ctx: &mut Context,
        assets: &Assets,
        draw_context: &DrawContext,
    ) -> GameResult<()> {
        let idle_tile = match idle_state.is_blinking {
            true => &assets.sprites.dino.blink,
            false => &assets.sprites.dino.idle,
        };

        graphics::draw(
            ctx,
            &assets.sprites.sprite_sheet,
            make_draw_param(idle_tile, draw_context.screen_scale, [100.0, 250.0]),
        )?;

        Ok(())
    }

    fn draw_walking(
        &self,
        walking_state: &WalkingState,
        ctx: &mut Context,
        assets: &Assets,
        draw_context: &DrawContext,
    ) -> GameResult<()> {
        let walk_tile = match walking_state.is_left_leg {
            true => &assets.sprites.dino.walk1,
            false => &assets.sprites.dino.walk2,
        };

        graphics::draw(
            ctx,
            &assets.sprites.sprite_sheet,
            make_draw_param(walk_tile, draw_context.screen_scale, [100.0, 250.0]),
        )?;

        Ok(())
    }
}
