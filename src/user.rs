use crate::physics::Position;
use specs::{Component, DenseVecStorage};

const SPEED: f64 = 0.5;
const SPEED_BOOST: f64 = 30.0;
const MAX_SPEED: f64 = 50.0;
const ROTATION: f64 = 0.02;

#[derive(Debug, Default)]
pub struct Camera {
    /// X position.
    x: f64,
    /// Y position.
    y: f64,
    /// Rotation.
    rot: f64,
    /// Speed.
    speed: f64,
}

impl Camera {
    /// Instantiate a new camera object.
    pub fn new() -> Self {
        Self {
            rot: 90.0,
            ..Default::default()
        }
    }

    /// Map normal coordinates to relative camera coordinates.
    pub fn map(&self, pos: &Position) -> (f64, f64) {
        (pos.x - self.x, pos.y - self.y)
    }

    /// Update the position according to the velocity and speed.
    pub fn update(&mut self, dt: f64) {
        self.x += self.rot.sin() * self.speed * dt;
        self.y += self.rot.cos() * self.speed * dt;
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
            self.rot += ROTATION;
        }
        if keys[3] {
            // D
            self.rot -= ROTATION;
        }
    }
}

#[derive(Component, Debug, Default)]
pub struct MovesWithCamera;
