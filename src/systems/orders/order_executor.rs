use crate::components::terrain::{Terrain, TILE_REAL_SIZE};
use crate::components::unit::Unit;
use crate::systems::orders::Orders::*;
use amethyst::core::ecs::*;
use amethyst::core::{Time, Transform};
use amethyst::prelude::*;

enum Action {
    Move((usize, usize)),
    Attack(Entity),
}

#[derive(Default)]
pub struct OrderExecutorSystem {}

impl<'s> System<'s> for OrderExecutorSystem {
    type SystemData = (
        WriteStorage<'s, Unit>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Terrain>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut units, mut transforms, mut terrain, entities, time): Self::SystemData) {
        let mut actions: Vec<Action> = Vec::new();

        for (unit, transform, entity) in (&units, &transforms, &*entities).join() {
            match unit.order {
                Attack(opponent) => {
                    let opp_pos = transforms.get(opponent).unwrap().translation();
                    let pos = transform.translation();

                    let distance =
                        (opp_pos.x - pos.x).abs().hypot((opp_pos.y - pos.y).abs()) * TILE_REAL_SIZE;

                    if distance <= unit.engagement_distance {
                    } else {
                    }

                    // opponent_transform.translation().x
                }
                Retreat => {}
                MoveTo((x, y)) => {}
                Hold => {}
                Ambush => {}
                DigIn => {}
                AwaitingOrders => {}
            };
        }
    }
}
