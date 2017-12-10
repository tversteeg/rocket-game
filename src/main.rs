extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod game;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
            "rocket-game",
            [600, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = game::Game::new(opengl);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            game.draw(&r);
        }

        if let Some(u) = e.update_args() {
            game.update(&u);
        }

        if let Some(b) = e.press_args() {
            game.press_key(b);
        }

        if let Some(b) = e.release_args() {
            game.release_key(b);
        }
    }
}
