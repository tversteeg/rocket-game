use crate::physics::{Position, Rotation};
use specs::{Component, DenseVecStorage};
use std::f64::consts::PI;

type Vec2 = vek::Vec2<f64>;

#[const_tweaker::tweak]
const SPEED: f64 = 0.5;
#[const_tweaker::tweak(min = 0.0, max = 100.0, step = 1.0)]
const SPEED_BOOST: f64 = 30.0;
#[const_tweaker::tweak(min = 0.0, max = 100.0, step = 1.0)]
const MAX_SPEED: f64 = 50.0;
#[const_tweaker::tweak(min = 0.0, max = 0.2, step = 0.01)]
const ROTATION: f64 = 0.02;

#[derive(Debug, Default)]
pub struct InputState {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl InputState {
    /// Instantiate a new keyboard state with nothing pressed.
    pub fn new() -> Self {
        Self {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }

    /// Set all the keyboard keys to not-pressed.
    pub fn reset(&mut self) {
        self.up = false;
        self.down = false;
        self.left = false;
        self.right = false;
    }

    /// Set the up key as pressed.
    pub fn set_up_pressed(&mut self) {
        self.up = true;
    }

    /// Get whether the up key is pressed or not.
    pub fn up_pressed(&self) -> bool {
        self.up
    }

    /// Set the down key as pressed.
    pub fn set_down_pressed(&mut self) {
        self.down = true;
    }

    /// Get whether the down key is pressed or not.
    pub fn down_pressed(&self) -> bool {
        self.down
    }

    /// Set the left key as pressed.
    pub fn set_left_pressed(&mut self) {
        self.left = true;
    }

    /// Get whether the left key is pressed or not.
    pub fn left_pressed(&self) -> bool {
        self.left
    }

    /// Set the right key as pressed.
    pub fn set_right_pressed(&mut self) {
        self.right = true;
    }

    /// Get whether the right key is pressed or not.
    pub fn right_pressed(&self) -> bool {
        self.right
    }
}

#[derive(Debug, Default)]
pub struct Camera {
    /// Absolute position.
    pos: Vec2,
    /// Camera center.
    pivot: Vec2,
    /// Relative rotation.
    rot: f64,
    /// Offset that's always added to the rotation.
    rot_offset: f64,
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
            rot_offset: PI / 4.0,
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
        rot.0 + self.rot + self.rot_offset
    }

    /// Update the position according to the velocity and speed.
    pub fn update(&mut self, dt: f64) {
        self.pos += Vec2::new(
            self.rot_sin * self.speed * dt,
            self.rot_cos * self.speed * dt,
        );
    }

    /// Hande keyboard and mouse input.
    pub fn handle_input(&mut self, input: &InputState) {
        if input.up_pressed() {
            // W
            // A bit of boost
            self.speed = (self.speed + *SPEED).min(*MAX_SPEED + *SPEED_BOOST);
        } else {
            // Remove the boost when the button isn't pressed down
            self.speed = self.speed.min(*MAX_SPEED);
        }
        if input.down_pressed() {
            // S
            self.speed = (self.speed - *SPEED).max(0.0);
        }
        if input.left_pressed() {
            // A
            self.rotate(*ROTATION);
        }
        if input.right_pressed() {
            // D
            self.rotate(-*ROTATION);
        }
    }

    /// Rotate the camera.
    fn rotate(&mut self, offset: f64) {
        self.rot += offset;

        // Only calculate the angles once
        let with_offset = self.rot + self.rot_offset;
        self.rot_sin = with_offset.sin();
        self.rot_cos = with_offset.cos();
    }
}

#[derive(Component, Debug, Default)]
pub struct MovesWithCamera;

#[derive(Component, Debug, Default)]
pub struct RotatesWithCamera;
