use crate::user::{Camera, MovesWithCamera, RotatesWithCamera};
use derive_deref::{Deref, DerefMut};
use specs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, WriteStorage};
use specs_blit::Sprite;
use std::time::Duration;

type Vec2 = vek::Vec2<f64>;

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

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }
}

#[derive(Component, Debug, Default)]
pub struct CartesianVelocity {
    pub rot: f64,
    pub speed: f64,
}

#[derive(Component, Debug, Default, Deref, DerefMut)]
pub struct Rotation(pub f64);

#[derive(Component, Debug, Default)]
pub struct RotationFollowsVelocity;

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
            pos.0 += vel.0 * dt;
        }
    }
}

pub struct CartesianVelocitySystem;
impl<'a> System<'a> for CartesianVelocitySystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, CartesianVelocity>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, (dt, vel, mut pos): Self::SystemData) {
        let dt = dt.to_seconds();

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.0.x += vel.rot.sin() * vel.speed * dt;
            pos.0.y += vel.rot.cos() * vel.speed * dt;
        }
    }
}

pub struct RotationSystem;
impl<'a> System<'a> for RotationSystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Rotation>,
        ReadStorage<'a, RotationFollowsVelocity>,
    );

    fn run(&mut self, (vel, mut rot, follow_rotation): Self::SystemData) {
        for (vel, rot, _) in (&vel, &mut rot, &follow_rotation).join() {
            // Point the rotation towards the velocity
            rot.0 = f64::atan2(vel.y, vel.x);
        }
    }
}

pub struct CartesianRotationSystem;
impl<'a> System<'a> for CartesianRotationSystem {
    type SystemData = (
        ReadStorage<'a, CartesianVelocity>,
        WriteStorage<'a, Rotation>,
        ReadStorage<'a, RotationFollowsVelocity>,
    );

    fn run(&mut self, (vel, mut rot, follow_rotation): Self::SystemData) {
        for (vel, rot, _) in (&vel, &mut rot, &follow_rotation).join() {
            // Point the rotation towards the velocity
            rot.0 = -vel.rot.to_degrees();
        }
    }
}

pub struct SpritePositionSystem;
impl<'a> System<'a> for SpritePositionSystem {
    type SystemData = (
        Read<'a, Camera>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, MovesWithCamera>,
    );

    fn run(&mut self, (camera, pos, mut sprite, moves_with_camera): Self::SystemData) {
        // Map the camera position when the entity moves with it
        for (pos, sprite, _) in (&pos, &mut sprite, &moves_with_camera).join() {
            let offset = camera.map_pos(&pos);
            sprite.set_pos(offset.x as i32, offset.y as i32);
        }
        // Just set the normal position when it's stationary
        for (pos, sprite, _) in (&pos, &mut sprite, !&moves_with_camera).join() {
            sprite.set_pos(pos.x as i32, pos.y as i32);
        }
    }
}

pub struct SpriteRotationSystem;
impl<'a> System<'a> for SpriteRotationSystem {
    type SystemData = (
        Read<'a, Camera>,
        ReadStorage<'a, Rotation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, RotatesWithCamera>,
    );

    fn run(&mut self, (camera, rot, mut sprite, rotates_with_camera): Self::SystemData) {
        // Map the camera rotation when the entity moves with it
        for (rot, sprite, _) in (&rot, &mut sprite, &rotates_with_camera).join() {
            sprite.set_rot((camera.map_rot(rot).to_degrees() + 90.0) as i16);
        }
        // Just set the rotation position when it's stationary
        for (rot, sprite, _) in (&rot, &mut sprite, !&rotates_with_camera).join() {
            sprite.set_rot((rot.to_degrees() + 90.0) as i16);
        }
    }
}
