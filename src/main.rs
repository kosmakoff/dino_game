extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rodio;

mod game;

use crate::piston::AdvancedWindow;
use crate::piston::PressEvent;
use game::Game;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;
use rodio::Source;
use std::io::BufReader;
use std::io::Cursor;
use std::io::Read;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Dino Game", [640, 480])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = Game::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
            let dt = args.dt;
            let fps = 1.0 / dt;
            let fps_string = format!("Dino game. FPS: {:.1}", fps);
            window.set_title(fps_string);
        }

        if let Some(Button::Keyboard(_)) = e.press_args() {
            let score_reached_data = include_bytes!("../assets/sounds/score-reached.ogg");
            let buf_reader = Cursor::new(score_reached_data.as_ref());
            let source = rodio::Decoder::new(buf_reader).unwrap();
            rodio::play_raw(&app.sound_device, source.convert_samples());
        }
    }
}
