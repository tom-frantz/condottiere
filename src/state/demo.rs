use crate::state::initialize_camera;
use crate::utils::sprites::load_sprite_sheet;

use crate::components::unit::{Team, Unit};
use crate::resources::map_registry::MapRegistry;
use crate::resources::vision_registry::VisionRegistry;
use crate::systems::orders::Orders;
use crate::utils::movement::get_real_location;
use amethyst::assets::Handle;
use amethyst::core::ecs::world::Generation;
use amethyst::core::Transform;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::palette::rgb::Rgb;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use std::num::NonZeroI32;

pub const PIXEL_SIZE: f32 = 2.0;
const GRID_SIZE: usize = 10;

const BUFFER: f32 = (PIXEL_SIZE * 3.0);
const CAMERA_DIMENSIONS: f32 = (GRID_SIZE as f32 * PIXEL_SIZE) + BUFFER;
const PIXEL_OFFSET: f32 = BUFFER / PIXEL_SIZE + (PIXEL_SIZE / 2.0);

#[derive(Default, Debug)]
pub struct DemoState {
    camera: Option<Entity>,
    sprites: Option<Handle<SpriteSheet>>,
}

impl DemoState {
    pub fn create_demo_state(&mut self, world: &mut World) {
        let mut map = Vec::with_capacity(GRID_SIZE.pow(2));
        let sprite = SpriteRender::new(self.sprites.clone().unwrap(), 0);

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let mut transform = Transform::default();
                transform.set_translation_xyz(
                    (x as f32 * PIXEL_SIZE),
                    (y as f32 * PIXEL_SIZE),
                    -1.0,
                );

                let mut tint = Tint::default();
                tint.0.color = Rgb::new(0.0, (x + y) as f32 / 20.0, 0.0);

                let tile = world
                    .create_entity()
                    .with(transform)
                    .with(sprite.clone())
                    .with(tint)
                    .build();

                map.push(tile);
            }
        }

        let player_unit = Unit::build_entity()
            .pos(1, 1)
            .hp(50)
            .team(Team::Player)
            .sprite(sprite.clone())
            .create(world);

        let enemy_unit = Unit::build_entity()
            .pos(8, 8)
            .team(Team::Enemy)
            .sprite(sprite)
            .create(world);

        {
            let mut units = world.write_storage::<Unit>();
            units
                .get_mut(player_unit)
                .unwrap()
                .set_objective(Orders::Attack(enemy_unit));
            units
                .get_mut(enemy_unit)
                .unwrap()
                .set_objective(Orders::Attack(player_unit));
        }
        world.insert(MapRegistry::new(GRID_SIZE, GRID_SIZE, map));
        world.insert(VisionRegistry::new())
    }
}

impl SimpleState for DemoState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let mut world = data.world;

        world.register::<Unit>();

        let camera = initialize_camera(world, (CAMERA_DIMENSIONS, CAMERA_DIMENSIONS), PIXEL_OFFSET);
        let sprites = load_sprite_sheet(world);

        self.camera = Some(camera);
        self.sprites = Some(sprites);

        self.create_demo_state(world);
    }
}
