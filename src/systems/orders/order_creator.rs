use crate::components::terrain::Terrain;
use crate::components::unit::Unit;
use amethyst::core::ecs::*;
use amethyst::core::Time;
use amethyst::prelude::*;

#[derive(Default)]
pub struct OrderCreatorSystem {}

impl<'s> System<'s> for OrderCreatorSystem {
    type SystemData = (
        WriteStorage<'s, Unit>,
        ReadStorage<'s, Terrain>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut units, mut terrain, entities, time): Self::SystemData) {
        for (unit, entity) in (&units, &*entities).join() {}
    }
}
