use crate::physics::*;

use anyhow::Result;
use rand::prelude::*;
use specs::{prelude::*, Component, DenseVecStorage};
use specs_blit::{
    blit::{BlitBuffer, Color},
    Sprite,
};
use sprite_gen::{gen_sprite, Options};

#[derive(Component, Debug, Default)]
pub struct Asteroid {}

pub fn spawn_asteroids(
    world: &mut World,
    amount: usize,
    width: usize,
    height: usize,
) -> Result<()> {
    let asteroid_mask = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 1, -1, 0, 0, 0, 1, 1, -1, 0, 0, 0, 1, 1,
        -1, 0, 0, 1, 1, 1, -1, 0, 1, 1, 1, 2, 2, 0, 1, 1, 1, 2, 2, 0, 1, 1, 1, 2, 2, 0, 1, 1, 1, 1,
        -1, 0, 1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1,
    ];
    let asteroid_mask_size = (6, 12);

    // Asteroids are mirrored on both axes
    let options = Options {
        mirror_x: true,
        mirror_y: true,
        colored: true,
        ..Options::default()
    };

    let mut rng = rand::thread_rng();

    for _ in 0..amount {
        // Generate the sprite
        let sprite = {
            let buf = BlitBuffer::from_buffer(
                &gen_sprite(&asteroid_mask, asteroid_mask_size.0, options),
                (asteroid_mask_size.0 * 2) as i32,
                Color::from_u32(0xFFFFFFFF),
            );

            specs_blit::load(buf)?
        };

        // Add the entity to the ECS system
        world
            .create_entity()
            .with(Asteroid::default())
            .with(Position {
                x: rng.gen_range(0, width) as f64,
                y: rng.gen_range(0, height) as f64,
            })
            .with(Velocity {
                x: rng.gen_range(-10.0, 10.0),
                y: rng.gen_range(-10.0, 10.0),
            })
            .with(Sprite::new(sprite))
            .build();
    }

    Ok(())
}
