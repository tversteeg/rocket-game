use crate::physics::Velocity;
use specs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, WriteStorage};

#[derive(Component, Debug, Default)]
pub struct KeyboardControlled;

pub struct KeyboardSystem;
impl<'a> System<'a> for KeyboardSystem {
    type SystemData = (
        Read<'a, Vec<bool>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, (keyboard, keyboard_controlled, mut vel): Self::SystemData) {
        for (vel, _) in (&mut vel, &keyboard_controlled).join() {
            if keyboard[0] {
                // W
                vel.y -= 0.2;
            } else if keyboard[1] {
                // A
                vel.x -= 0.2;
            } else if keyboard[2] {
                // S
                vel.y += 0.2;
            } else if keyboard[3] {
                // D
                vel.x += 0.2;
            }
        }
    }
}
