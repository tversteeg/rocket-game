use anyhow::Result;
use line_drawing::Bresenham;
use specs::{prelude::*, Component, DenseVecStorage};
use specs_blit::{
    blit::{BlitBuffer, Color},
    PixelBuffer, SpriteRef,
};
use sprite_gen::{gen_sprite, MaskValue, Options};

type Vec2 = vek::Vec2<f64>;

/// Draw a line.
#[derive(Component, Debug)]
pub struct Line {
    pub p1: Vec2,
    pub p2: Vec2,
    pub color: u32,
}

impl Line {
    /// Create a new line which casts from a starting point to a direction.
    pub fn from_direction(pos: &Vec2, dir: f64, length: f64, color: u32) -> Self {
        let x = pos.x + dir.sin() * length;
        let y = pos.y + dir.cos() * length;

        Self {
            p1: *pos,
            p2: Vec2::new(x, y),
            color,
        }
    }
}

pub struct LineSystem;
impl<'a> System<'a> for LineSystem {
    type SystemData = (Write<'a, PixelBuffer>, ReadStorage<'a, Line>);

    fn run(&mut self, (mut pixels, line): Self::SystemData) {
        let width = pixels.width() as i32;
        let height = pixels.height() as i32;
        let pixels = pixels.pixels_mut();

        for line in line.join() {
            for (x, y) in Bresenham::new(
                (line.p1.x as i32, line.p1.y as i32),
                (line.p2.x as i32, line.p2.y as i32),
            ) {
                if x >= width || y >= height {
                    break;
                }

                pixels[x as usize + y as usize * width as usize] = line.color;
            }
        }
    }
}

/// Generate a random sprite from a mask and return it as a blit buffer.
pub fn generate(
    width: usize,
    options: Options,
    mask: &[MaskValue],
    rotations: u16,
) -> Result<SpriteRef> {
    let buffer_width = if options.mirror_x { width * 2 } else { width };

    let buf = BlitBuffer::from_buffer(
        &gen_sprite(&mask, width, options)
            .into_iter()
            // Invert the colors
            .map(|p| p ^ 0xFF_FF_FF_FF)
            .collect::<Vec<_>>(),
        buffer_width as i32,
        Color::from_u32(0),
    );

    specs_blit::load(buf, rotations)
}
