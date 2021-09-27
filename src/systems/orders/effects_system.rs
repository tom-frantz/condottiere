use crate::components::projectile::Projectile;
use crate::components::unit::{DamageResult, Unit};
use crate::resources::PIXEL_SIZE;
use crate::systems::orders::Orders;
use crate::systems::rendering::new_renders::RenderEvents;
use crate::utils::movement::Map2d;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{Entities, Entity, ReadExpect, ReaderId, WriteExpect, WriteStorage};
use amethyst::core::Transform;
use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{System, SystemData, World},
};

#[derive(Debug, Clone)]
pub enum EffectEvents {
    Move(Entity, Map2d),
    Damage(Entity, Entity, f32),
}

#[derive(Debug, SystemDesc)]
#[system_desc(name(EffectsSystemDesc))]
pub struct EffectsSystem {
    #[system_desc(event_channel_reader)]
    effects_system_event_id: ReaderId<EffectEvents>,
    #[system_desc(event_channel_reader)]
    render_system_event_id: ReaderId<RenderEvents>,
}

impl EffectsSystem {
    fn new(
        effects_system_event_id: ReaderId<EffectEvents>,
        render_system_event_id: ReaderId<RenderEvents>,
    ) -> Self {
        EffectsSystem {
            effects_system_event_id,
            render_system_event_id,
        }
    }
}

impl<'s> System<'s> for EffectsSystem {
    type SystemData = (
        ReadExpect<'s, EventChannel<EffectEvents>>,
        WriteExpect<'s, EventChannel<RenderEvents>>,
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Unit>,
    );

    fn run(
        &mut self,
        (effect_events, mut render_events, mut entities, mut transforms, mut units): Self::SystemData,
    ) {
        for event in effect_events.read(&mut self.effects_system_event_id) {
            match event {
                EffectEvents::Move(entity, new_pos) => {
                    let unit_translation = transforms.get_mut(entity.clone()).unwrap();
                    unit_translation.set_translation_x(new_pos.0 * PIXEL_SIZE);
                    unit_translation.set_translation_y(new_pos.1 * PIXEL_SIZE);
                }
                EffectEvents::Damage(source, target, damage) => {
                    let source_transform = transforms.get(source.clone()).unwrap();
                    let target_transform = transforms.get(target.clone()).unwrap();

                    println!("damage {}", damage);

                    match units.get_mut(target.clone()).unwrap().take_damage(*damage) {
                        DamageResult::Dead => {
                            entities.delete(target.clone());
                            units.get_mut(source.clone()).unwrap().objective =
                                Orders::AwaitingOrders;
                        }
                        DamageResult::Alive => {}
                    };

                    render_events.single_write(RenderEvents::Projectile(Projectile::new(
                        source_transform.into(),
                        target_transform.into(),
                        20.0,
                    )));
                }
            }
        }
    }
}
