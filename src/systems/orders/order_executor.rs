use crate::components::projectile::Projectile;
use crate::components::terrain::{Terrain, TILE_REAL_SIZE};
use crate::components::unit::Unit;
use crate::resources::sprites_registry::SpriteRegistry;
use crate::systems::orders::Orders::*;
use crate::utils::camera::CameraHeight;
use crate::utils::movement::Map2d;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::*;
use amethyst::core::math::Vector3;
use amethyst::core::{Time, Transform};
use amethyst::prelude::*;
use amethyst::renderer::sprite::Sprites;
use amethyst::renderer::{Camera, SpriteRender};

enum Action {
    Move((usize, usize)),
    Attack(Entity),
}

#[derive(Default)]
pub struct OrderExecutorSystem {
    last_push: f32,
}

impl OrderExecutorSystem {}

impl<'s> System<'s> for OrderExecutorSystem {
    type SystemData = (
        WriteStorage<'s, Unit>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Projectile>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, SpriteRegistry>,
        ReadStorage<'s, Terrain>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            mut units,
            mut transforms,
            mut projectiles,
            mut sprites,
            sprite_registry,
            terrain,
            entities,
            time,
        ): Self::SystemData,
    ) {
        self.last_push += time.delta_seconds();
        let add_graphics = if self.last_push >= 1.0 {
            self.last_push -= 1.0;
            true
        } else {
            false
        };

        let mut projectiles_entities: Vec<Projectile> = Vec::new();

        for (unit, transform, entity) in (&units, &transforms, &*entities).join() {
            match unit.order {
                Attack(opponent) => {
                    let opponent_pos = transforms.get(opponent).unwrap();
                    if add_graphics {
                        let projectile = Projectile::new(
                            Map2d::from_transform(transform),
                            Map2d::from_transform(opponent_pos),
                            3.0 * 2.0 * 2.0,
                        );
                        projectiles_entities.push(projectile)
                    }
                }
                Retreat => {}
                MoveTo((x, y)) => {}
                Hold => {}
                Ambush => {}
                DigIn => {}
                AwaitingOrders => {}
            };
        }

        let sprite = sprite_registry.get_default_sprite();

        for projectile in projectiles_entities {
            let mut transform = Transform::default();
            transform.set_translation_xyz(
                projectile.start.0,
                projectile.start.1,
                CameraHeight::Projectiles as u8 as f32,
            );
            transform.set_scale(Vector3::new(0.5, 0.5, 1.0));

            entities
                .build_entity()
                .with(projectile, &mut projectiles)
                .with(transform, &mut transforms)
                .with(sprite.clone(), &mut sprites)
                .build();
        }
    }
}
