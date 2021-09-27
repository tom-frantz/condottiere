use crate::components::projectile::Projectile;
use crate::resources::sprites_registry::SpriteRegistry;
use crate::utils::camera::CameraHeight;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Entities, Read, ReaderId, WriteStorage};
use amethyst::core::math::Vector3;
use amethyst::core::{Time, Transform};
use amethyst::renderer::SpriteRender;
use amethyst::shred::WriteExpect;
use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{System, SystemData, World},
};

const UPDATE_TIME: f32 = 0.2;

#[derive(Debug)]
pub enum RenderEvents {
    Projectile(Projectile),
}

#[derive(SystemDesc)]
#[system_desc(name(RenderSystemDesc))]
pub struct RenderSystem {
    #[system_desc(event_channel_reader)]
    render_events_reader_id: ReaderId<RenderEvents>,
    #[system_desc(skip)]
    last_update: f32,
}

impl RenderSystem {
    pub fn new(render_events_reader_id: ReaderId<RenderEvents>) -> Self {
        RenderSystem {
            render_events_reader_id,
            last_update: 0.0,
        }
    }
}

impl<'s> System<'s> for RenderSystem {
    type SystemData = (
        Read<'s, EventChannel<RenderEvents>>,
        Read<'s, Time>,
        Entities<'s>,
        WriteExpect<'s, SpriteRegistry>,
        WriteStorage<'s, Projectile>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (
            render_events,
            time,
            entities,
            sprite_registry,
            mut projectiles,
            mut sprites,
            mut transforms,
        ): Self::SystemData,
    ) {
        let delta = time.delta_seconds();
        self.last_update += delta;
        let update_sprites = if self.last_update > UPDATE_TIME {
            self.last_update -= UPDATE_TIME;
            true
        } else {
            false
        };
        for event in render_events.read(&mut self.render_events_reader_id) {
            match event {
                RenderEvents::Projectile(projectile) => {
                    if !update_sprites {
                        continue;
                    }
                    println!("Update_sprite! {}", self.last_update);
                    let mut transform = projectile
                        .start
                        .to_transform(CameraHeight::Projectiles as u8 as f32);

                    transform.set_scale(Vector3::new(0.5, 0.5, 1.0));

                    entities
                        .build_entity()
                        .with(projectile.clone(), &mut projectiles)
                        .with(sprite_registry.get_default_sprite(), &mut sprites)
                        .with(transform, &mut transforms)
                        .build();
                }
            };
        }
    }
}
