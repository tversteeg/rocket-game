use piston::input::*;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct Game {
    gl: GlGraphics,
    rotation: f64,

    x: f64,
    y: f64,

    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool
}

impl Game {
    pub fn new(opengl: OpenGL) -> Self {
        Game {
            gl: GlGraphics::new(opengl),
            rotation: 0.0,

            x: 0.0,
            y: 0.0,

            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }

    pub fn press_key(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => {
                self.up_pressed = true;
            }
            Button::Keyboard(Key::Down) => {
                self.down_pressed = true;
            }
            Button::Keyboard(Key::Left) => {
                self.left_pressed = true;
            }
            Button::Keyboard(Key::Right) => {
                self.right_pressed = true;
            }
            _ => {}
        }
    }

    pub fn release_key(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => {
                self.up_pressed = false;
            }
            Button::Keyboard(Key::Down) => {
                self.down_pressed = false;
            }
            Button::Keyboard(Key::Left) => {
                self.left_pressed = false;
            }
            Button::Keyboard(Key::Right) => {
                self.right_pressed = false;
            }
            _ => {}
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.rotation += 2.0 * args.dt;

        if self.up_pressed {
            self.y -= 50.0 * args.dt;
        }
        if self.down_pressed {
            self.y += 50.0 * args.dt;
        }
        if self.left_pressed {
            self.x -= 50.0 * args.dt;
        }
        if self.right_pressed {
            self.x += 50.0 * args.dt;
        }
    }

    pub fn draw(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (self.x, self.y);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y)
                                       .rot_rad(rotation)
                                       .trans(-25.0, -25.0);

            rectangle(RED, square, transform, gl);
        });
    }
}
