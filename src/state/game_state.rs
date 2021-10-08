use crate::utils::camera::initialize_camera;
use crate::utils::sprites::load_sprite_sheet;

use crate::components::terrain::Terrain;
use crate::components::unit::{Team, Unit};
use crate::resources::map_registry::MapRegistry;
use crate::resources::vision_registry::VisionRegistry;
use crate::resources::*;
use crate::systems::orders::Orders;

use crate::components::equipment::Equipment;
use crate::resources::camera::CameraEntity;
use crate::resources::sprites_registry::SpriteRegistry;
use crate::resources::ui::UiRegistry;
use crate::state::helpers::equipment::get_equipment;
use amethyst::assets::Handle;
use amethyst::ecs::Entity;
use amethyst::input::{InputEvent, StringBindings};
use amethyst::prelude::*;
use amethyst::renderer::rendy::wsi::winit::Event;
use amethyst::renderer::{SpriteRender, SpriteSheet};
use amethyst::ui::{UiEvent, UiEventType};

#[derive(Default, Debug)]
pub struct GameState {
    camera: Option<Entity>,
    sprites: Option<Handle<SpriteSheet>>,
}

impl GameState {
    pub fn create_demo_state(&mut self, world: &mut World) {
        let sprite = SpriteRender::new(self.sprites.clone().unwrap(), 0);
        world.register::<Equipment>();

        {
            let world_map_registry = MapRegistry::new(GRID_SIZE, GRID_SIZE, world, sprite.clone());
            let terrain = world.read_storage::<Terrain>();
            let world_vision_registry = VisionRegistry::new(&world_map_registry, terrain);
            let ui_registry = UiRegistry::default();

            world.insert(world_vision_registry);
            world.insert(world_map_registry);
            world.insert(SpriteRegistry::new(sprite.clone()));
            world.insert(ui_registry);
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

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Unit>();

        let camera = initialize_camera(world, (CAMERA_DIMENSIONS, CAMERA_DIMENSIONS), PIXEL_OFFSET);
        let sprites = load_sprite_sheet(world);

        let camera_resource =
            CameraEntity::new(camera.clone(), CAMERA_DIMENSIONS, CAMERA_DIMENSIONS);
        self.camera = Some(camera);
        self.sprites = Some(sprites);

        world.insert(camera_resource);
        self.create_demo_state(world);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        // println!("{:?}", event);
        match event {
            StateEvent::Window(windo) => match windo {
                Event::WindowEvent { .. } => {}
                Event::DeviceEvent { .. } => {}
                Event::Awakened => {}
                Event::Suspended(_) => {}
            },
            StateEvent::Ui(ref ui) => match &ui.event_type {
                UiEventType::Click => {}
                UiEventType::ClickStart => {
                    println!("{:?}", &event)
                }
                UiEventType::ClickStop => {}
                UiEventType::HoverStart => {
                    println!("{:?}", &event)
                }
                UiEventType::HoverStop => {}
                UiEventType::Dragging { .. } => {}
                UiEventType::Dropped { .. } => {}
                UiEventType::ValueChange => {}
                UiEventType::ValueCommit => {}
                UiEventType::Focus => {}
                UiEventType::Blur => {}
            },
            StateEvent::Input(state_event) => match state_event {
                InputEvent::KeyPressed { .. } => {}
                InputEvent::KeyReleased { .. } => {}
                InputEvent::KeyTyped(_) => {}
                InputEvent::MouseButtonPressed(ref e) => {}
                InputEvent::MouseButtonReleased(_) => {}
                InputEvent::ButtonPressed(_) => {}
                InputEvent::ButtonReleased(_) => {}
                InputEvent::CursorMoved { .. } => {}
                InputEvent::MouseMoved { .. } => {}
                InputEvent::MouseWheelMoved(_) => {}
                InputEvent::AxisMoved { .. } => {}
                InputEvent::ControllerAxisMoved { .. } => {}
                InputEvent::ControllerButtonPressed { .. } => {}
                InputEvent::ControllerButtonReleased { .. } => {}
                InputEvent::ControllerConnected { .. } => {}
                InputEvent::ControllerDisconnected { .. } => {}
                InputEvent::ActionPressed(_) => {}
                InputEvent::ActionReleased(_) => {}
                InputEvent::ActionWheelMoved(_) => {}
            },
        };
        Trans::None
        // todo!()
    }
}
