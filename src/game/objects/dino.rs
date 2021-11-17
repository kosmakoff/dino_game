use crate::game::{make_draw_param, Assets, DrawContext, GRAVITY, GROUND_LEVEL, JUMP_VELOCITY};
use ggez::{
    graphics,
    input::keyboard::{KeyCode, KeyMods},
    timer, Context, GameResult,
};

const BLINKING_DURATION: u128 = 100_000;
const BLINKING_INTERVAL: u128 = 2_500_000;

const WALKING_STEP_DURATION: u128 = 100_000;

const DINO_VELOCITY: f32 = 300.0;

struct IdleState {
    since: u128,
    is_blinking: bool,
}

impl IdleState {
    fn new() -> Self {
        Self {
            since: 0,
            is_blinking: false,
        }
    }
}

struct WalkingState {
    since: u128,
    is_left_leg: bool,
}

impl WalkingState {
    fn new() -> Self {
        Self {
            since: 0,
            is_left_leg: false,
        }
    }

    fn new_with_since_and_leg(since: u128, is_left_leg: bool) -> Self {
        Self { since, is_left_leg }
    }

    fn new_with_leg(is_left_leg: bool) -> Self {
        Self {
            since: 0,
            is_left_leg,
        }
    }
}

struct CrouchedState {
    since: u128,
    is_left_leg: bool,
}

impl CrouchedState {
    fn new(since: u128, is_left_leg: bool) -> Self {
        Self { since, is_left_leg }
    }
}

struct JumpingState {
    vertical_velocity: f32,
    vertical_position: f32,
    is_left_leg: bool,
}

impl JumpingState {
    fn new(is_left_leg: bool) -> Self {
        Self {
            vertical_velocity: JUMP_VELOCITY,
            vertical_position: 0.0,
            is_left_leg,
        }
    }
}

enum DinoState {
    Idle(IdleState),
    Walking(WalkingState),
    Jumping(JumpingState),
    Crouched(CrouchedState),
    Hit,
}

impl DinoState {
    pub fn new() -> Self {
        DinoState::Idle(IdleState::new())
    }
}

pub struct Dino(DinoState);

impl Dino {
    pub fn new() -> Self {
        Dino(DinoState::new())
    }

    pub(crate) fn update(&mut self, ctx: &mut Context) {
        let dt = timer::delta(ctx);
        let dt_micros = dt.as_micros();
        let dt_float = dt.as_secs_f32();

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

                // steps alternation
                if walking.since >= WALKING_STEP_DURATION {
                    walking.since -= WALKING_STEP_DURATION;
                    walking.is_left_leg = !walking.is_left_leg;
                }
            }

            DinoState::Jumping(ref mut jumping) => {
                jumping.vertical_velocity -= dt_float * GRAVITY;
                jumping.vertical_position += dt_float * jumping.vertical_velocity;

                if jumping.vertical_position < 0.0 {
                    // transition back to walking
                    let walking = WalkingState::new_with_leg(jumping.is_left_leg);
                    *dino_state = DinoState::Walking(walking);
                }
            }

            DinoState::Crouched(ref mut crouched) => {
                crouched.since += dt_micros;

                // steps alternation
                if crouched.since >= WALKING_STEP_DURATION {
                    crouched.since -= WALKING_STEP_DURATION;
                    crouched.is_left_leg = !crouched.is_left_leg;
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

            DinoState::Jumping(ref jumping) => {
                self.draw_jumping(jumping, ctx, assets, draw_context)?
            }

            DinoState::Crouched(ref crouched) => {
                self.draw_crouched(crouched, ctx, assets, draw_context)?
            }

            DinoState::Hit => self.draw_hit(ctx, assets, draw_context)?,
        };

        Ok(())
    }

    pub fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Space => {
                // depending on current state - start walking or jump
                let Dino(ref mut dino_state) = self;

                match dino_state {
                    DinoState::Idle(_) => {
                        // start walking
                        let walking_state = WalkingState::new();
                        *dino_state = DinoState::Walking(walking_state);
                    }
                    DinoState::Walking(ref walking_state) => {
                        // make jump
                        let jumping_state = JumpingState::new(walking_state.is_left_leg);
                        *dino_state = DinoState::Jumping(jumping_state);
                    }
                    _ => (),
                };

                // jump

                // play sound
            }

            KeyCode::Down => {
                let Dino(ref mut dino_state) = self;

                if let DinoState::Walking(ref walking_state) = dino_state {
                    let crouched_state =
                        CrouchedState::new(walking_state.since, walking_state.is_left_leg);
                    *dino_state = DinoState::Crouched(crouched_state);
                }
            }
            _ => (),
        };
    }

    pub fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        match keycode {
            KeyCode::Down => {
                let Dino(ref mut dino_state) = self;

                if let DinoState::Crouched(ref crouched_state) = dino_state {
                    let walking_state = WalkingState::new_with_since_and_leg(
                        crouched_state.since,
                        crouched_state.is_left_leg,
                    );
                    *dino_state = DinoState::Walking(walking_state);
                }
            }
            _ => (),
        }
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
            make_draw_param(idle_tile, draw_context.screen_scale, [100.0, GROUND_LEVEL]),
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
            make_draw_param(walk_tile, draw_context.screen_scale, [100.0, GROUND_LEVEL]),
        )?;

        Ok(())
    }

    fn draw_jumping(
        &self,
        jumping_state: &JumpingState,
        ctx: &mut Context,
        assets: &Assets,
        draw_context: &DrawContext,
    ) -> GameResult<()> {
        let jump_tile = match jumping_state.is_left_leg {
            true => &assets.sprites.dino.walk1,
            false => &assets.sprites.dino.walk2,
        };

        graphics::draw(
            ctx,
            &assets.sprites.sprite_sheet,
            make_draw_param(
                jump_tile,
                draw_context.screen_scale,
                [100.0, GROUND_LEVEL - jumping_state.vertical_position],
            ),
        )?;

        Ok(())
    }

    fn draw_crouched(
        &self,
        crouched_state: &CrouchedState,
        ctx: &mut Context,
        assets: &Assets,
        draw_context: &DrawContext,
    ) -> GameResult<()> {
        let crouched_tile = match crouched_state.is_left_leg {
            true => &assets.sprites.dino.crouched1,
            false => &assets.sprites.dino.crouched2,
        };

        graphics::draw(
            ctx,
            &assets.sprites.sprite_sheet,
            make_draw_param(
                crouched_tile,
                draw_context.screen_scale,
                [100.0, GROUND_LEVEL],
            ),
        )?;

        Ok(())
    }

    fn draw_hit(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        draw_context: &DrawContext,
    ) -> GameResult<()> {
        let hit_tile = &assets.sprites.dino.hit;

        graphics::draw(
            ctx,
            &assets.sprites.sprite_sheet,
            make_draw_param(hit_tile, draw_context.screen_scale, [100.0, GROUND_LEVEL]),
        )
    }

    pub(crate) fn velocity(&self) -> f32 {
        let Dino(ref dino_state) = self;

        match dino_state {
            DinoState::Idle(_) => 0.0,
            DinoState::Walking(_) => DINO_VELOCITY,
            DinoState::Jumping(_) => DINO_VELOCITY,
            DinoState::Crouched(_) => DINO_VELOCITY,
            DinoState::Hit => 0.0,
        }
    }
}
