use amethyst::core::Transform;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::Camera;

pub mod demo;

pub fn initialize_camera(world: &mut World, dimensions: (f32, f32), offset: f32) -> Entity {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        dimensions.0 / 2.0 - offset,
        dimensions.1 / 2.0 - offset,
        1.0,
    );

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.0, dimensions.1))
        .with(transform)
        .build()
}
