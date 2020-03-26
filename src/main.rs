mod asteroid;
mod audio;
mod physics;
mod projectile;
mod rocket;
mod sprite;
mod user;

use crate::{
    asteroid::*,
    audio::Audio,
    physics::*,
    projectile::{Laser, LaserLifetimeSystem, Lifetime, LifetimeSystem},
    rocket::*,
    sprite::{Line, LineSystem},
    user::*,
};
use anyhow::Result;
use pixels::{wgpu::Surface, Pixels, SurfaceTexture};
use safe_transmute::to_bytes;
use specs::prelude::*;
use specs_blit::{PixelBuffer, RenderSystem, Sprite};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

type Vec2 = vek::Vec2<f64>;

const WIDTH: usize = 1200;
const HEIGHT: usize = 800;

fn main() -> Result<()> {
    // Setup the ECS system
    let mut world = World::new();

    // Load the game components
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Rotation>();
    world.register::<CartesianVelocity>();
    world.register::<RotationFollowsVelocity>();
    world.register::<Lifetime>();
    world.register::<Asteroid>();
    world.register::<Laser>();
    world.register::<Rocket>();
    world.register::<MovesWithCamera>();
    world.register::<RotatesWithCamera>();
    world.register::<Line>();

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

    // Add the audio system
    world.insert(Audio::new());

    // Spawn the initial asteroids
    spawn_asteroids(&mut world, 20, WIDTH, HEIGHT)?;

    // Spawn the initial rockets
    spawn_small_rockets(&mut world, 20, WIDTH, HEIGHT)?;

    // Spawn the player rocket
    spawn_rocket(&mut world, WIDTH / 2, HEIGHT / 2)?;

    // Setup the dispatcher with the blit system
    let mut dispatcher = DispatcherBuilder::new()
        .with(LifetimeSystem, "lifetime", &[])
        .with(LaserLifetimeSystem, "laser_lifetime", &[])
        .with(LineSystem, "line", &["laser_lifetime"])
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
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rocket Game")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(WIDTH as u32, HEIGHT as u32, surface);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    {
        // Start the audio
        let mut audio = world.write_resource::<Audio>();
        audio.run();
    }

    // Add the tweaking gui
    const_tweaker::run().expect("Could not run server");

    event_loop.run(move |event, _, control_flow| {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                // Close button was pressed
                *control_flow = ControlFlow::Exit
            }
            Event::MainEventsCleared => {
                // Application update code

                {
                    // Clear the buffer
                    let mut buffer = world.write_resource::<PixelBuffer>();
                    buffer.clear(0);
                }

                {
                    // Update the camera
                    let mut camera = world.write_resource::<Camera>();
                    camera.handle_input(
                        &world.read_resource::<InputState>(),
                        &mut world.write_resource::<Audio>(),
                    );
                    camera.update(world.read_resource::<DeltaTime>().to_seconds());
                }

                // Update specs
                dispatcher.dispatch(&world);

                // Add/remove entities added in dispatch through `LazyUpdate`
                world.maintain();

                // Queue a RedrawRequested event.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Get the pixel buffer resource to render it
                let buffer = world.read_resource::<PixelBuffer>();

                // Copy the source buffer into the pixels array
                // Source is u32, make 4x u8 from it
                let transmuted = to_bytes::transmute_to_bytes(buffer.pixels());
                pixels.get_frame().copy_from_slice(transmuted);

                // Draw the pixels
                pixels.render();
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(new_size),
                ..
            } => {
                pixels.resize(new_size.width, new_size.height);
            }
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                // Handle keyboard input

                // Match WASD & Dvorak (Comma, A, O, E)
                if let KeyboardInput {
                    virtual_keycode: Some(virtual_code),
                    state,
                    ..
                } = input
                {
                    match virtual_code {
                        VirtualKeyCode::W | VirtualKeyCode::Comma => {
                            let mut input_state = world.write_resource::<InputState>();
                            input_state.set_up_state(state == ElementState::Pressed);
                        }
                        VirtualKeyCode::A => {
                            let mut input_state = world.write_resource::<InputState>();
                            input_state.set_left_state(state == ElementState::Pressed);
                        }
                        VirtualKeyCode::S | VirtualKeyCode::O => {
                            let mut input_state = world.write_resource::<InputState>();
                            input_state.set_down_state(state == ElementState::Pressed);
                        }
                        VirtualKeyCode::D | VirtualKeyCode::E => {
                            let mut input_state = world.write_resource::<InputState>();
                            input_state.set_right_state(state == ElementState::Pressed);
                        }
                        VirtualKeyCode::Space => {
                            if state == ElementState::Pressed {
                                projectile::shoot_laser(
                                    &mut world,
                                    &Vec2::new(WIDTH as f64 / 2.0, HEIGHT as f64 / 2.0),
                                    90.0,
                                    500.0,
                                );
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    });
}
