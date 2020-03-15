mod asteroid;
mod physics;
mod rocket;
mod sprite;
mod user;

use crate::{asteroid::*, physics::*, rocket::*, user::*};

use anyhow::Result;
use minifb::Key;
use specs::prelude::*;
use specs_blit::{PixelBuffer, RenderSystem, Sprite};

use std::time::Duration;

type Vec2 = vek::Vec2<f64>;

const WIDTH: usize = 600;
const HEIGHT: usize = 400;

fn main() -> Result<()> {
    // Add the tweaking gui
    #[cfg(debug_assertions)]
    const_tweaker::run().expect("Could not run server");

    // Setup the ECS system
    let mut world = World::new();

    // Load the game components
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Rotation>();
    world.register::<CartesianVelocity>();
    world.register::<RotationFollowsVelocity>();
    world.register::<Asteroid>();
    world.register::<Rocket>();
    world.register::<MovesWithCamera>();
    world.register::<RotatesWithCamera>();

    // Load the sprite rendering component
    world.register::<Sprite>();

    // Add the pixel buffer as a resource so it can be accessed from the RenderSystem later, to be
    // updated every frame
    world.insert(PixelBuffer::new(WIDTH, HEIGHT));

    // Add the deltatime to calculate the physics, to be updated every frame
    world.insert(DeltaTime::new(1.0 / 60.0));

    // Add the current keyboard state, to be updated every frame
    world.insert(InputState::new());

    // Add the camera
    world.insert(Camera::new(Vec2::new(
        WIDTH as f64 / 2.0,
        HEIGHT as f64 / 2.0,
    )));

    // Spawn the initial asteroids
    spawn_asteroids(&mut world, 20, WIDTH, HEIGHT)?;

    // Spawn the initial rockets
    spawn_small_rockets(&mut world, 20, WIDTH, HEIGHT)?;

    // Spawn the player rocket
    spawn_rocket(&mut world, WIDTH / 2, HEIGHT / 2)?;

    // Setup the dispatcher with the blit system
    let mut dispatcher = DispatcherBuilder::new()
        .with(CartesianVelocitySystem, "cartesian_velocity", &[])
        .with(VelocitySystem, "velocity", &[])
        .with(RotationSystem, "rotation", &["velocity"])
        .with(
            CartesianRotationSystem,
            "cartesian_rotation",
            &["cartesian_velocity"],
        )
        .with(
            SpriteRotationSystem,
            "sprite_rotation",
            &["rotation", "cartesian_rotation"],
        )
        .with(
            SpritePositionSystem,
            "sprite_position",
            &["velocity", "cartesian_velocity"],
        )
        .with_thread_local(RenderSystem)
        .build();

    // Setup the window
    let window_options = minifb::WindowOptions {
        scale: minifb::Scale::X2,
        ..minifb::WindowOptions::default()
    };
    let mut window = minifb::Window::new("Rocket Game", WIDTH, HEIGHT, window_options)?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        {
            // Clear the buffer
            let mut buffer = world.write_resource::<PixelBuffer>();
            buffer.clear(0);
        }

        // Get which keys are pressed
        if let Some(keys) = window.get_keys() {
            let mut input = world.write_resource::<InputState>();
            input.reset();

            for t in keys {
                match t {
                    // Qwerty or Dvorak
                    Key::W | Key::Comma => input.set_up_pressed(),
                    Key::A => input.set_left_pressed(),
                    Key::S | Key::O => input.set_down_pressed(),
                    Key::D | Key::E => input.set_right_pressed(),
                    _ => (),
                }
            }
        }

        {
            // Update the camera
            let mut camera = world.write_resource::<Camera>();
            camera.handle_input(&world.read_resource::<InputState>());
            camera.update(world.read_resource::<DeltaTime>().to_seconds());
        }

        // Update specs
        dispatcher.dispatch(&world);

        // Add/remove entities added in dispatch through `LazyUpdate`
        world.maintain();

        // Get the pixel buffer resource to render it
        let buffer = world.read_resource::<PixelBuffer>();
        // Render the pixel buffer
        window.update_with_buffer(&buffer.pixels(), buffer.width(), buffer.height())?;
    }

    Ok(())
}
