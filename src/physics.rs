use specs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, WriteStorage};
use specs_blit::Sprite;
use std::{f64::consts::PI, time::Duration};

#[derive(Default)]
pub struct DeltaTime(pub Duration);

impl DeltaTime {
    pub fn new(time: f64) -> Self {
        DeltaTime(Duration::from_millis((time * 1000.0) as u64))
    }

    pub fn to_seconds(&self) -> f64 {
        self.0.as_secs() as f64 + self.0.subsec_nanos() as f64 * 1e-9
    }
}

#[derive(Component, Debug, Default)]
pub struct RotationFollowsVelocity;

#[derive(Component, Debug, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Component, Debug, Default)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

pub struct VelocitySystem;
impl<'a> System<'a> for VelocitySystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (dt, vel, mut pos): Self::SystemData) {
        let dt = dt.to_seconds();

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * dt;
            pos.y += vel.y * dt;
        }
    }
}

pub struct SpritePositionSystem;
impl<'a> System<'a> for SpritePositionSystem {
    type SystemData = (ReadStorage<'a, Position>, WriteStorage<'a, Sprite>);

    fn run(&mut self, (pos, mut sprite): Self::SystemData) {
        for (pos, sprite) in (&pos, &mut sprite).join() {
            sprite.set_pos(pos.x as i32, pos.y as i32);
        }
    }
}

pub struct RotationSystem;
impl<'a> System<'a> for RotationSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, RotationFollowsVelocity>,
    );

    fn run(&mut self, (vel, mut sprite, follow_rotation): Self::SystemData) {
        for (vel, sprite, _) in (&vel, &mut sprite, &follow_rotation).join() {
            let rotation_in_degrees = f64::atan2(vel.y, vel.x) * 180.0 / PI;
            sprite.set_rot(((rotation_in_degrees + 360.0) + 90.0) as i16)
        }
    }
}
