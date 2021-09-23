use crate::components::projectile::Projectile;
use crate::utils::movement::Map2d;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::*;
use amethyst::core::{Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::prelude::*;

#[derive(Debug, Default)]
pub struct ProjectileSystem {}

impl<'s> System<'s> for ProjectileSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Projectile>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, mut projectiles, time, entities): Self::SystemData) {
        let delta = time.delta_seconds();

        // Handle shooting them
        for (transform, projectile, entity) in (&mut transforms, &projectiles, &*entities).join() {
            let next = projectile.next(delta, Map2d::from_transform(transform));

            match next {
                None => {
                    entities.delete(entity);
                }
                Some(point) => {
                    transform.set_translation_x(point.0);
                    transform.set_translation_y(point.1);
                }
            };
        }
    }
}
