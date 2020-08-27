mod game;

use game::Game;
use ggez::event::{self};
use ggez::{graphics, ContextBuilder, GameResult};

fn main() -> GameResult {
    let mut window_mode = ggez::conf::WindowMode::default();
    window_mode.width = 640.0;
    window_mode.height = 480.0;

    let (ctx, event_loop) = &mut ContextBuilder::new("Dino Game", "Oleg Kosmakov")
        .window_mode(window_mode)
        .build()?;

    let game = &mut Game::new(ctx)?;
    event::run(ctx, event_loop, game)
}
