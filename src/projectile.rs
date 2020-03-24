use crate::{audio::Audio, physics::DeltaTime, sprite::Line};
use specs::{prelude::*, Component, DenseVecStorage};

type Vec2 = vek::Vec2<f64>;

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

/// Shoot a laser by spawning a new laser object.
pub fn shoot_laser(world: &mut World, pos: &Vec2, dir: f64, strength: f64) {
    // Create the laser entity
    world
        .create_entity()
        .with(Laser)
        // The lifetime of the laser depends on the strength
        .with(Lifetime(0.02))
        .with(Line::from_direction(
            pos,
            dir.to_radians(),
            strength,
            0xFF_00_00,
        ))
        .build();

    // Play a laser sound
    {
        let mut audio = world.write_resource::<Audio>();
        audio.play_laser();
    }
}
