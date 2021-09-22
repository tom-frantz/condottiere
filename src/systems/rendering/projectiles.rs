use crate::components::projectile::Projectile;
use crate::utils::movement::Map2d;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::*;
use amethyst::core::{Time, Transform};
use amethyst::derive::SystemDesc;
use amethyst::prelude::*;

#[derive(Debug, SystemDesc)]
#[system_desc(name(ProjectileSystemDesc))]
pub struct ProjectileSystem {
    #[system_desc(event_channel_reader)]
    reader_id: ReaderId<ProjectileEvents>,
}

#[derive(Debug)]
pub enum ProjectileEvents {
    Create(Projectile),
}

impl ProjectileSystem {
    pub fn new(reader_id: ReaderId<ProjectileEvents>) -> Self {
        Self { reader_id }
    }
}

impl<'s> System<'s> for ProjectileSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Projectile>,
        Read<'s, EventChannel<ProjectileEvents>>,
        Read<'s, Time>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, mut projectiles, events, time, entities): Self::SystemData) {
        let delta = time.delta_seconds();

        // TODO fix the initialization so this doesn't break
        for event in events.read(&mut self.reader_id) {
            match event {
                ProjectileEvents::Create(projectile) => {
                    let mut transform = Transform::default();
                    transform.set_translation_xyz(projectile.start.0, projectile.start.1, 2.0);
                    entities
                        .build_entity()
                        .with(transform, &mut transforms)
                        // TODO fix this clone here?
                        .with((*projectile).clone(), &mut projectiles)
                        // .with()
                        .build();
                }
            }
        }

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
