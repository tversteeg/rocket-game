use crate::{
    physics::*,
    sprite::generate,
    user::{MovesWithCamera, RotatesWithCamera},
};
use anyhow::Result;
use rand::prelude::*;
use specs::{prelude::*, Component, DenseVecStorage};
use specs_blit::Sprite;
use sprite_gen::{MaskValue::*, Options};

#[derive(Component, Debug, Default)]
pub struct Rocket {}

pub fn spawn_small_rockets(
    world: &mut World,
    amount: usize,
    screen_width: usize,
    screen_height: usize,
) -> Result<()> {
    let (width, _height, options) = (
        6,
        12,
        Options {
            mirror_x: true,
            mirror_y: false,
            colored: true,
            edge_brightness: 0.3,
            color_variations: 0.2,
            brightness_noise: 0.3,
            saturation: 0.5,
        },
    );
    let rocket_mask = [
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Empty,
        Empty, Empty, Empty, Body1, Solid, Empty, Empty, Empty, Empty, Body1, Solid, Empty, Empty,
        Empty, Body1, Body1, Solid, Empty, Empty, Body1, Body1, Body2, Solid, Empty, Empty, Body1,
        Body2, Body2, Solid, Empty, Body1, Body1, Body2, Body2, Solid, Empty, Body1, Body2, Body2,
        Body2, Solid, Empty, Body1, Body2, Body2, Body1, Solid, Empty, Body1, Body1, Body1, Body1,
        Body1, Empty, Body1, Body1, Empty, Body1, Body1,
    ];

    let mut rng = rand::thread_rng();

    for _ in 0..amount {
        // Generate the sprite
        let sprite = generate(width, options, &rocket_mask, 16)?;

        // Add the entity to the ECS system
        world
            .create_entity()
            .with(Rocket::default())
            .with(Position::new(
                rng.gen_range(0, screen_width) as f64,
                rng.gen_range(0, screen_height) as f64,
            ))
            .with(Velocity::new(
                rng.gen_range(-10.0, 10.0),
                rng.gen_range(-10.0, 10.0),
            ))
            .with(Rotation(0.0))
            .with(RotationFollowsVelocity)
            .with(MovesWithCamera)
            .with(RotatesWithCamera)
            .with(Sprite::new(sprite))
            .build();
    }

    Ok(())
}

pub fn spawn_rocket(world: &mut World, x: usize, y: usize) -> Result<()> {
    let (width, _height, options) = (
        11,
        24,
        Options {
            mirror_x: true,
            mirror_y: false,
            colored: true,
            edge_brightness: 0.14,
            color_variations: 0.2,
            brightness_noise: 1.00,
            saturation: 0.0,
        },
    );
    let rocket_mask = [
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Body1, Body1, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Body1, Body1, Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1,
        Body1, Body1, Solid, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body1,
        Solid, Empty, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1, Solid, Empty,
        Empty, Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body2, Solid, Empty, Empty, Empty,
        Empty, Empty, Body1, Body1, Body1, Body1, Body2, Solid, Empty, Empty, Empty, Empty, Empty,
        Body1, Body1, Body1, Body2, Body2, Body2, Empty, Empty, Empty, Empty, Empty, Body1, Body1,
        Body2, Body2, Body2, Body2, Empty, Empty, Empty, Empty, Empty, Body1, Body1, Solid, Body2,
        Body2, Body2, Empty, Empty, Body1, Empty, Empty, Body1, Body1, Solid, Body2, Body2, Body2,
        Empty, Empty, Body1, Body1, Body1, Body1, Body2, Solid, Body2, Body2, Body1, Empty, Body1,
        Body2, Body2, Body1, Body1, Body2, Solid, Body2, Body2, Body1, Empty, Body1, Solid, Body2,
        Body2, Body1, Body1, Body2, Body2, Body2, Body1, Empty, Body1, Body2, Body1, Body2, Body2,
        Body2, Body2, Body2, Body2, Body1, Empty, Body1, Body1, Empty, Body1, Body1, Body1, Body2,
        Body2, Body2, Body1, Empty, Body1, Body1, Empty, Empty, Body1, Body1, Body1, Body2, Body1,
        Body1, Empty, Body1, Body1, Empty, Empty, Empty, Body1, Body1, Body2, Body2, Body2, Empty,
        Empty, Empty, Empty, Empty, Body1, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty,
        Empty, Empty, Body1, Body1, Body1, Body1, Body1, Body1, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty,
        Empty, Empty, Empty, Empty,
    ];

    // Generate the sprite
    let sprite = generate(width, options, &rocket_mask, 45)?;

    // Add the entity to the ECS system
    world
        .create_entity()
        .with(Rocket::default())
        .with(Position::new(x as f64, y as f64))
        .with(Rotation(0.0))
        .with(CartesianVelocity {
            speed: 0.0,
            rot: 0.0,
        })
        .with(Sprite::new(sprite))
        .build();

    Ok(())
}
