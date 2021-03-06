use crate::utils::camera::initialize_camera;
use crate::utils::sprites::load_sprite_sheet;

use crate::components::terrain::Terrain;
use crate::components::unit::{Team, Unit};
use crate::resources::map_registry::MapRegistry;
use crate::resources::vision_registry::VisionRegistry;
use crate::resources::*;
use crate::systems::orders::Orders;

use crate::components::equipment::Equipment;
use crate::resources::sprites_registry::SpriteRegistry;
use crate::state::helpers::equipment::get_equipment;
use amethyst::assets::Handle;
use amethyst::ecs::Entity;
use amethyst::prelude::*;
use amethyst::renderer::{SpriteRender, SpriteSheet};

#[derive(Default, Debug)]
pub struct DemoState {
    camera: Option<Entity>,
    sprites: Option<Handle<SpriteSheet>>,
}

impl DemoState {
    pub fn create_demo_state(&mut self, world: &mut World) {
        let sprite = SpriteRender::new(self.sprites.clone().unwrap(), 0);
        world.register::<Equipment>();

        {
            let world_map_registry = MapRegistry::new(GRID_SIZE, GRID_SIZE, world, sprite.clone());
            let terrain = world.read_storage::<Terrain>();
            let world_vision_registry = VisionRegistry::new(&world_map_registry, terrain);

            world.insert(world_vision_registry);
            world.insert(world_map_registry);
            world.insert(SpriteRegistry::new(sprite.clone()))
        }

        {
            let (_, _, pukka_1, pukka_2, _, _) = get_equipment();

            let player_unit = Unit::build_entity()
                .pos(1, 1)
                .hp(1000.0)
                .team(Team::Player)
                .sprite(sprite.clone())
                .equipment(Equipment::new(pukka_2, 100))
                .create(world);
            let enemy_unit = Unit::build_entity()
                .pos(8, 8)
                .hp(1000.0)
                .team(Team::Enemy)
                .sprite(sprite)
                .equipment(Equipment::new(pukka_1, 100))
                .create(world);

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
    }
}

impl SimpleState for DemoState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Unit>();

        let camera = initialize_camera(world, (CAMERA_DIMENSIONS, CAMERA_DIMENSIONS), PIXEL_OFFSET);
        let sprites = load_sprite_sheet(world);

        self.camera = Some(camera);
        self.sprites = Some(sprites);

        self.create_demo_state(world);
    }
}
