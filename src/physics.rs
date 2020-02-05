use specs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, WriteStorage};
use std::time::Duration;

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
