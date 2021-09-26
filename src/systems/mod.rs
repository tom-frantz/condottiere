use crate::systems::orders::effects_system::EffectEvents;
use crate::systems::rendering::new_renders::RenderEvents;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{DispatcherBuilder, World};
use amethyst::core::SystemBundle;
use amethyst::prelude::*;
use amethyst::Error;

pub mod orders;
pub mod rendering;

#[derive(Debug, Default)]
pub struct SystemResourceBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SystemResourceBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let mut render_events_channel = EventChannel::<RenderEvents>::new();
        let mut effects_events_channel = EventChannel::<EffectEvents>::new();

        world.insert(render_events_channel);
        world.insert(effects_events_channel);
        // System that adds `ApplicationResource` to the `World`
        // builder.add(MySystem.build(world), "my_system", &[]);
        Ok(())
    }
}
