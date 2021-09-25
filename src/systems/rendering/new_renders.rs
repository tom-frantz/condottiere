use crate::components::projectile::Projectile;
use crate::resources::sprites_registry::SpriteRegistry;
use crate::utils::camera::CameraHeight;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Entities, Read, ReaderId, Write, WriteStorage};
use amethyst::core::math::Vector3;
use amethyst::core::Transform;
use amethyst::renderer::sprite::Sprites;
use amethyst::renderer::SpriteRender;
use amethyst::shred::WriteExpect;
use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{System, SystemData, World},
};

#[derive(Debug)]
pub enum RenderEvents {
    Projectile(Projectile),
}

#[derive(SystemDesc)]
#[system_desc(name(RenderSystemDesc))]
pub struct RenderSystem {
    #[system_desc(event_channel_reader)]
    render_events_reader_id: ReaderId<RenderEvents>,
}

impl RenderSystem {
    pub fn new(render_events_reader_id: ReaderId<RenderEvents>) -> Self {
        RenderSystem {
            render_events_reader_id,
        }
    }
}

impl<'s> System<'s> for RenderSystem {
    type SystemData = (
        Read<'s, EventChannel<RenderEvents>>,
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
            mut entities,
            mut sprite_registry,
            mut projectiles,
            mut sprites,
            mut transforms,
        ): Self::SystemData,
    ) {
        for event in render_events.read(&mut self.render_events_reader_id) {
            match event {
                RenderEvents::Projectile(projectile) => {
                    let mut transform = projectile
                        .start
                        .to_transform(CameraHeight::Projectiles as u8 as f32);

                    transform.set_scale(Vector3::new(0.5, 0.5, 1.0));

                    entities
                        .build_entity()
                        .with(projectile.clone(), &mut projectiles)
                        .with(sprite_registry.get_default_sprite(), &mut sprites)
                        .with(transform, &mut transforms)
                        .build()
                }
            };
        }
    }
}
