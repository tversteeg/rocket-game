extern crate ggez;

use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::Color;
use ggez::timer;
use std::env;
use std::path;

struct MainState {
    a: i32,
    direction: i32,
    image: graphics::Image,
}

impl MainState {
    fn new(context: &mut Context) -> GameResult<MainState> {
        context.print_resource_stats();

        let image = graphics::Image::new(context, "/rocket.png").unwrap();

        let state = MainState {
            a: 0,
            direction: 1,
            image: image
        };

        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        const DESIRED_FPS: u32 = 60;

        while timer::check_update_time(context, DESIRED_FPS) {
            // Update game ..
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::set_color(context, Color::from((128, 128, 128, 255)))?;
        graphics::clear(context);

        let draw_param = graphics::DrawParam {
            dest: graphics::Point2::new(100.0, 100.0),
            offset: graphics::Point2::new(0.5, 0.5),
            ..Default::default()
        };
        graphics::draw_ex(context, &self.image, draw_param)?;

        graphics::present(context);

        timer::yield_now();
        Ok(())
    }
}

pub fn main() {
    let config = conf::Conf::new();

    let context = &mut Context::load_from_conf("rocket-game", "ggez", config).unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        
        context.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new(context).unwrap();
    if let Err(e) = event::run(context, state) {
        println!("Error encountered: {}", e);
    }
}
