use crate::physics::{Position, Rotation};
use specs::{Component, DenseVecStorage};

type Vec2 = vek::Vec2<f64>;

const SPEED: f64 = 0.5;
const SPEED_BOOST: f64 = 30.0;
const MAX_SPEED: f64 = 50.0;
const ROTATION: f64 = 0.02;

#[derive(Debug, Default)]
pub struct Camera {
    /// Absolute position.
    pos: Vec2,
    /// Camera center.
    pivot: Vec2,
    rot: f64,
    /// Calculated sin that only needs to be calculated once.
    rot_sin: f64,
    /// Calculated cos that only needs to be calculated once.
    rot_cos: f64,
    /// Speed.
    speed: f64,
}

impl Camera {
    /// Instantiate a new camera object.
    pub fn new(pivot: Vec2) -> Self {
        let mut c = Self {
            pivot,
            ..Default::default()
        };

        c.rotate(0.0);

        c
    }

    /// Map normal coordinates to relative camera coordinates.
    pub fn map_pos(&self, pos: &Position) -> Vec2 {
        let delta = pos.0 - self.pos - self.pivot;

        let new = Vec2::new(
            delta.x * self.rot_cos - delta.y * self.rot_sin,
            delta.x * self.rot_sin + delta.y * self.rot_cos,
        );

        new + self.pivot
    }

    /// Map normal rotation with camera rotation.
    pub fn map_rot(&self, rot: &Rotation) -> f64 {
        rot.0 + self.rot
    }

    /// Update the position according to the velocity and speed.
    pub fn update(&mut self, dt: f64) {
        self.pos += Vec2::new(
            self.rot_sin * self.speed * dt,
            self.rot_cos * self.speed * dt,
        );
    }

    /// Move by keyboard.
    pub fn handle_keyboard(&mut self, keys: &[bool]) {
        if keys[0] {
            // W
            // A bit of boost
            self.speed = (self.speed + SPEED).min(MAX_SPEED + SPEED_BOOST);
        } else {
            // Remove the boost when the button isn't pressed down
            self.speed = self.speed.min(MAX_SPEED);
        }
        if keys[2] {
            // S
            self.speed = (self.speed - SPEED).max(0.0);
        }
        if keys[1] {
            // A
            self.rotate(ROTATION);
        }
        if keys[3] {
            // D
            self.rotate(-ROTATION);
        }
    }

    /// Rotate the camera.
    fn rotate(&mut self, offset: f64) {
        self.rot += offset;

        self.rot_sin = self.rot.sin();
        self.rot_cos = self.rot.cos();
    }
}

#[derive(Component, Debug, Default)]
pub struct MovesWithCamera;

#[derive(Component, Debug, Default)]
pub struct RotatesWithCamera;
