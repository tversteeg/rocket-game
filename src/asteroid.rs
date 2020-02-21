use crate::physics::*;

use crate::sprite::generate;
use anyhow::Result;
use rand::prelude::*;
use specs::{prelude::*, Component, DenseVecStorage};
use specs_blit::Sprite;
use sprite_gen::{MaskValue::*, Options};

#[derive(Component, Debug, Default)]
pub struct Asteroid {}

pub fn spawn_asteroids(
    world: &mut World,
    amount: usize,
    screen_width: usize,
    screen_height: usize,
) -> Result<()> {
    let (width, _height, options) = (
        11,
        11,
        Options {
            mirror_x: false,
            mirror_y: false,
            colored: true,
            edge_brightness: 0.3,
            color_variations: 0.2,
            brightness_noise: 0.3,
            saturation: 0.5,
        },
    );
    let asteroid_mask = [
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1,
        Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Solid,
        Body1, Body1, Body1, Empty, Empty, Empty, Body1, Body1, Body1, Solid, Solid, Solid, Body1,
        Body1, Body1, Empty, Empty, Body1, Body1, Solid, Solid, Solid, Solid, Solid, Body1, Body1,
        Empty, Empty, Body1, Body1, Body1, Solid, Solid, Solid, Body1, Body1, Body1, Empty, Empty,
        Empty, Body1, Body1, Body1, Solid, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty,
        Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1,
        Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty,
    ];

    let mut rng = rand::thread_rng();

    for _ in 0..amount {
        // Generate the sprite
        let sprite = generate(width, options, &asteroid_mask)?;

        // Add the entity to the ECS system
        world
            .create_entity()
            .with(Asteroid::default())
            .with(Position {
                x: rng.gen_range(0, screen_width) as f64,
                y: rng.gen_range(0, screen_height) as f64,
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
