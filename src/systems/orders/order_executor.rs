use crate::components::equipment::Equipment;
use crate::components::unit::Unit;
use crate::systems::orders::effects_system::EffectEvents;
use crate::systems::orders::Orders;
use crate::systems::orders::Orders::*;
use crate::utils::movement::Map2d;
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::*;
use amethyst::core::{Time, Transform};
use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{System, SystemData, World},
};

enum Action {
    Move((usize, usize)),
    Attack(Entity),
}

#[derive(Debug, SystemDesc)]
#[system_desc(name(OrderExecutorSystemDesc))]
pub struct OrderExecutorSystem {
    #[system_desc(skip)]
    last_push: f32,
    #[system_desc(event_channel_reader)]
    effects_system_event_id: ReaderId<EffectEvents>,
}

impl OrderExecutorSystem {
    fn new(effects_system_event_id: ReaderId<EffectEvents>) -> Self {
        OrderExecutorSystem {
            last_push: 0.0,
            effects_system_event_id,
        }
    }
}

impl<'s> System<'s> for OrderExecutorSystem {
    type SystemData = (
        WriteStorage<'s, Unit>,
        WriteExpect<'s, EventChannel<EffectEvents>>,
        ReadStorage<'s, Equipment>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut units, mut events, equipment_components, mut transforms, entities, time): Self::SystemData,
    ) {
        let delta = time.delta_seconds();

        for (unit, equipment, transform, entity) in
            (&mut units, &equipment_components, &transforms, &*entities).join()
        {
            match &mut unit.objective {
                Attack(opponent) => events.single_write(EffectEvents::Damage(
                    entity,
                    opponent.clone(),
                    equipment.deal_damage() * delta,
                )),
                Retreat => {}
                MoveTo(p) => {
                    let append_vector = p.move_by(unit.speed * delta);
                    if let Some(vector) = append_vector {
                        events.single_write(EffectEvents::Move(
                            entity,
                            Map2d::from(transform) + vector,
                        ));
                        unit.objective = Orders::AwaitingOrders;
                    }
                }
                Hold => {}
                Ambush => {}
                DigIn => {}
                AwaitingOrders => {}
            };
        }
    }
}

#[cfg(test)]
mod test {
    use crate::utils::movement::Direction;

    #[test]
    fn testing() {
        let x: f64 = -1.0;
        let y: f64 = -1.0;
        let angle_bracket = 45.0;
        let tolerance = angle_bracket / 2.0;

        let degrees = x.atan2(y) * 180.0 / std::f64::consts::PI;
        let dir = ((degrees + tolerance).rem_euclid(360.0) / angle_bracket);
        println!("{:?}", dir);
        println!("{:?}", Direction::from_angle(x, y));
    }
}
