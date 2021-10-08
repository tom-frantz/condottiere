use crate::systems::orders::effects_system::EffectEvents;
use crate::systems::rendering::new_renders::RenderEvents;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::{DispatcherBuilder, World};
use amethyst::core::SystemBundle;
use amethyst::Error;

pub mod orders;
pub mod rendering;
pub mod utils;

#[derive(Debug, Default)]
pub struct SystemResourceBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for SystemResourceBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        let render_events_channel = EventChannel::<RenderEvents>::new();
        let effects_events_channel = EventChannel::<EffectEvents>::new();

        world.insert(render_events_channel);
        world.insert(effects_events_channel);
        Ok(())
    }
}
