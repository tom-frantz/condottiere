use crate::components::projectile::Projectile;
use crate::resources::PIXEL_SIZE;
use crate::utils::movement::Map2d;
use amethyst::core::ecs::*;
use amethyst::core::{Time, Transform};

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
        for (transform, projectile, entity) in
            (&mut transforms, &mut projectiles, &*entities).join()
        {
            let next = projectile.next(delta, Map2d::from_transform(transform));

            match next {
                None => {
                    entities.delete(entity).unwrap();
                }
                Some(point) => {
                    transform.set_translation_x(point.0 * PIXEL_SIZE);
                    transform.set_translation_y(point.1 * PIXEL_SIZE);
                }
            };
        }
    }
}
