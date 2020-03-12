use crate::physics::CartesianVelocity;
use specs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, WriteStorage};

const SPEED: f64 = 0.5;
const SPEED_BOOST: f64 = 30.0;
const MAX_SPEED: f64 = 50.0;
const ROTATION: f64 = 0.02;

#[derive(Component, Debug, Default)]
pub struct KeyboardControlled;

pub struct KeyboardSystem;
impl<'a> System<'a> for KeyboardSystem {
    type SystemData = (
        Read<'a, Vec<bool>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, CartesianVelocity>,
    );

    fn run(&mut self, (keyboard, keyboard_controlled, mut vel): Self::SystemData) {
        for (vel, _) in (&mut vel, &keyboard_controlled).join() {
            if keyboard[0] {
                // W
                // A bit of boost
                vel.speed = (vel.speed + SPEED).min(MAX_SPEED + SPEED_BOOST);
            } else {
                // Remove the boost when the button isn't pressed down
                vel.speed = vel.speed.min(MAX_SPEED);
            }
            if keyboard[2] {
                // S
                vel.speed = (vel.speed - SPEED).max(0.0);
            }
            if keyboard[1] {
                // A
                vel.rot += ROTATION;
            }
            if keyboard[3] {
                // D
                vel.rot -= ROTATION;
            }
        }
    }
}
