mod game;

use game::Game;
use ggez::event::{self};
use ggez::{ContextBuilder, GameResult};

use std::env;
use std::path;

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let mut window_mode = ggez::conf::WindowMode::default();
    window_mode.width = 1024.0;
    window_mode.height = 768.0;

    let (mut ctx, event_loop) = ContextBuilder::new("Dino Game", "Oleg Kosmakov")
        .window_mode(window_mode)
        .add_resource_path(resource_dir)
        .build()?;

    let game = Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}
