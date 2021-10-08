use amethyst::core::ecs::{Builder, Entity, World, WorldExt};
use amethyst::core::Transform;
use amethyst::renderer::Camera;

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum CameraHeight {
    // Bottom
    Terrain = 0,
    Units,
    Projectiles,
    // Top
}

pub fn _some() {
    // let vision_registry = &*world.read_resource::<VisionRegistry>();
    // let map_registry = &*world.read_resource::<MapRegistry>();

    // let vision_point: (usize, usize) = (5, 5);
    //
    // let visible = vision_registry.get_visible(vision_point).unwrap();
    //
    // let mut tints = world.write_component::<Tint>();
    //
    // visible
    //     .iter()
    //     .map(|(x, y)| {
    //         // println!("{:?} {:?}", x, y);
    //         map_registry.get_tile(*x, *y).unwrap().0.clone()
    //     })
    //     .for_each(|(entity)| {
    //         let tint = tints.get_mut(entity).unwrap();
    //         tint.0.blue = 1.0;
    //         tint.0.green = 1.0;
    //     });
    //
    // let mut vis_point = tints
    //     .get_mut(
    //         map_registry
    //             .get_tile(vision_point.0, vision_point.1)
    //             .unwrap()
    //             .0
    //             .clone(),
    //     )
    //     .unwrap()
    //     .0;
    // vis_point.red = 0.0;
    // vis_point.green = 0.0;
    // vis_point.blue = 0.0;
}

pub fn initialize_camera(world: &mut World, dimensions: (f32, f32), offset: f32) -> Entity {
    // Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        dimensions.0 / 2.0 - offset,
        dimensions.1 / 2.0 - offset,
        1000.0,
    );

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.0, dimensions.1))
        .with(transform)
        .build()
}
