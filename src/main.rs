use anyhow::Result;
use specs::prelude::*;
use specs_blit::{PixelBuffer, RenderSystem, Sprite};
use std::{thread::sleep, time::Duration};

mod asteroid;
mod physics;
mod rocket;

const WIDTH: usize = 600;
const HEIGHT: usize = 400;

fn main() -> Result<()> {
    // Setup the ECS system
    let mut world = World::new();

    // Load the sprite rendering component
    world.register::<Sprite>();

    // Add the pixel buffer as a resource so it can be accessed from the RenderSystem later
    world.insert(PixelBuffer::new(WIDTH, HEIGHT));

    // Setup the dispatcher with the blit system
    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(RenderSystem)
        .build();

    // Setup the window
    let window_options = minifb::WindowOptions {
        scale: minifb::Scale::X2,
        ..minifb::WindowOptions::default()
    };
    let mut window = minifb::Window::new("Rocket Game", WIDTH, HEIGHT, window_options)?;

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // Update specs
        dispatcher.dispatch(&world);

        // Add/remove entities added in dispatch through `LazyUpdate`
        world.maintain();

        // Get the pixel buffer resource to render it
        let buffer = world.read_resource::<PixelBuffer>();
        // Render the pixel buffer
        window.update_with_buffer(&buffer.pixels(), buffer.width(), buffer.height())?;

        // Don't use 100% CPU
        sleep(Duration::from_millis(12));
    }

    Ok(())
}
