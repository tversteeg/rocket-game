#[macro_use]
extern crate gfx;
extern crate ggez;

use ggez::graphics::*;
use ggez::mint::*;
use ggez::*;
use std::env;
use std::path;

gfx_defines! {
    constant VectorShaderConsts {
        ignore: f32 = "ignore",
    }
}

const DFIELD_VERTEX_SHADER_SOURCE: &[u8] = include_bytes!("dfield.vert");
const DFIELD_FRAGMENT_SHADER_SOURCE: &[u8] = include_bytes!("dfield.frag");

struct MainState {
    image: Image,

    shader_consts: VectorShaderConsts,
    shader: Shader<VectorShaderConsts>,
}

impl MainState {
    fn new(context: &mut Context) -> GameResult<MainState> {
        let mut image = Image::new(context, "/rocket.png").unwrap();
        image.set_filter(FilterMode::Linear);

        let shader_consts = VectorShaderConsts { ignore: 0.0 };

        let shader = Shader::from_u8(
            context,
            DFIELD_VERTEX_SHADER_SOURCE,
            DFIELD_FRAGMENT_SHADER_SOURCE,
            shader_consts,
            "VectorShaderConsts",
            None,
        )
        .unwrap();

        let state = MainState {
            image,
            shader_consts,
            shader,
        };

        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _context: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::set_default_filter(context, FilterMode::Linear);
        graphics::clear(context, Color::from((255, 255, 255, 255)));
        {
            let _lock = graphics::use_shader(context, &self.shader);
            self.shader.send(context, self.shader_consts)?;

            let draw_param = DrawParam {
                dest: Point2 { x: 300.0, y: 300.0 },
                offset: Point2 { x: 0.5, y: 0.5 },
                scale: Vector2 { x: 1.0, y: 3.0 },
                ..Default::default()
            };
            graphics::draw(context, &self.image, draw_param)?;
        }

        graphics::present(context)?;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");

        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (context, event_loop) = &mut ContextBuilder::new("rocket-game", "tversteeg")
        .add_resource_path(resource_dir)
        .build()?;

    let state = &mut MainState::new(context).unwrap();
    event::run(context, event_loop, state)
}
