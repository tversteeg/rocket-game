use crate::{audio::Audio, physics::DeltaTime, sprite::Line};
use specs::{prelude::*, Component, DenseVecStorage};

type Vec2 = vek::Vec2<f64>;

#[const_tweaker::tweak(min = 0.0, max = 1.0, step = 0.0001)]
const LASER_LIFETIME: f64 = 0.1;
#[const_tweaker::tweak(min = 0.0, max = 5000.0, step = 1.0)]
const LASER_DISSIPATION_FACTOR: f64 = 3000.0;

/// A laser beam.
#[derive(Component, Debug)]
pub struct Laser;

/// Object that will be destroyed when the time is up.
#[derive(Component, Debug, Default)]
pub struct Lifetime(pub f64);

pub struct LifetimeSystem;
impl<'a> System<'a> for LifetimeSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, DeltaTime>,
        WriteStorage<'a, Lifetime>,
    );

    fn run(&mut self, (entities, dt, mut lifetime): Self::SystemData) {
        let dt = dt.to_seconds();

        for (entity, lifetime) in (&entities, &mut lifetime).join() {
            lifetime.0 -= dt;

            // Kill the entity when the time runs out
            if lifetime.0 <= 0.0 {
                let _ = entities.delete(entity);
            }
        }
    }
}

pub struct LaserLifetimeSystem;
impl<'a> System<'a> for LaserLifetimeSystem {
    type SystemData = (
        ReadStorage<'a, Laser>,
        ReadStorage<'a, Lifetime>,
        WriteStorage<'a, Line>,
    );

    fn run(&mut self, (laser, lifetime, mut line): Self::SystemData) {
        for (_, lifetime, line) in (&laser, &lifetime, &mut line).join() {
            // Reduce the color depending on the lifetime left
            let color_byte = (lifetime.0 * *LASER_DISSIPATION_FACTOR).min(255.0).max(0.0) as u32;
            line.color = (color_byte & 0xF) << 16 | (color_byte & 0xF) << 8 | color_byte;
        }
    }
}

/// Shoot a laser by spawning a new laser object.
pub fn shoot_laser(world: &mut World, pos: &Vec2, dir: f64, strength: f64) {
    // Create the laser entity
    world
        .create_entity()
        .with(Laser)
        // The lifetime of the laser depends on the strength
        .with(Lifetime(*LASER_LIFETIME))
        .with(Line::from_direction(
            pos,
            dir.to_radians(),
            strength,
            0xFF_FF_FF,
        ))
        .build();

    // Play a laser sound
    {
        let mut audio = world.write_resource::<Audio>();
        audio.play_laser();
    }
}
