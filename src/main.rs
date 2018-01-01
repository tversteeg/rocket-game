#[macro_use]
extern crate gfx;
extern crate ggez;

use ggez::*;
use ggez::graphics::*;
use std::env;
use std::path;

gfx_defines!{
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
        context.print_resource_stats();

        let image = Image::new(context, "/rocket.png").unwrap();

        let shader_consts = VectorShaderConsts {
            ignore: 0.0
        };

        let shader = Shader::from_u8(context, DFIELD_VERTEX_SHADER_SOURCE, DFIELD_FRAGMENT_SHADER_SOURCE, shader_consts, "VectorShaderConsts", None).unwrap();

        let state = MainState {
            image: image,

            shader_consts: shader_consts,
            shader: shader
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
        graphics::set_default_filter(context, FilterMode::Linear);

        graphics::set_color(context, Color::from((128, 128, 128, 255)))?;
        graphics::clear(context);

        {
            let _lock = graphics::use_shader(context, &self.shader);
            self.shader.send(context, self.shader_consts)?;

            let draw_param = DrawParam {
                dest: Point2::new(300.0, 300.0),
                offset: Point2::new(0.5, 0.5),
                scale: Point2::new(1.0, 3.0),
                ..Default::default()
            };
            graphics::draw_ex(context, &self.image, draw_param)?;
        }

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
