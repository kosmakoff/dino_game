mod game;

use game::Game;
use ggez::event::{self};
use ggez::{graphics, ContextBuilder, GameResult};

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("Dino Game", "Oleg Kosmakov").build()?;

    graphics::set_drawable_size(ctx, 640.0, 480.0)?;

    let game = &mut Game::new(ctx)?;
    event::run(ctx, event_loop, game)
}
