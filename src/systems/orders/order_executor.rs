use crate::components::equipment::Equipment;
use crate::components::projectile::Projectile;
use crate::components::terrain::{Terrain, TILE_REAL_SIZE};
use crate::components::unit::Unit;
use crate::resources::sprites_registry::SpriteRegistry;
use crate::systems::orders::effects_system::EffectEvents;
use crate::systems::orders::Orders;
use crate::systems::orders::Orders::*;
use crate::systems::rendering::new_renders::RenderEvents;
use crate::utils::camera::CameraHeight;
use crate::utils::movement::{Direction, Map2d};
use amethyst::core::ecs::shrev::EventChannel;
use amethyst::core::ecs::*;
use amethyst::core::math::Vector3;
use amethyst::core::{Time, Transform};
use amethyst::prelude::*;
use amethyst::renderer::sprite::Sprites;
use amethyst::renderer::{Camera, SpriteRender};
use amethyst::{
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{System, SystemData, World},
};
use std::ops::Sub;

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
        self.last_push += delta;
        let add_graphics = if self.last_push >= 1.0 {
            self.last_push -= 1.0;
            true
        } else {
            false
        };

        let mut projectiles_entities: Vec<Projectile> = Vec::new();

        for (unit, equipment, transform, entity) in
            (&mut units, &equipment_components, &transforms, &*entities).join()
        {
            match &mut unit.objective {
                Attack(opponent) => {
                    events.single_write(EffectEvents::Damage(
                        entity,
                        opponent.clone(),
                        equipment.deal_damage(),
                    ))
                    // let opponent_pos = transforms.get(opponent.clone()).unwrap();
                    // if add_graphics {
                    //     events.single_write(RenderEvents::Projectile(Projectile::new(
                    //         transform.into(),
                    //         opponent_pos.into(),
                    //         5.0,
                    //     )))
                    // }
                    //     (Map2d::from_transform(opponent_pos) - Map2d::from_transform(transform));
                    // let opponent_pos = transforms.get(opponent).unwrap();
                    // let delta_pos =
                    //
                    // let range = equipment.equipment.stats().range;
                    //
                    // if delta_pos.magnitude() * TILE_REAL_SIZE <= range {
                    //     unit.objective = Orders::Attack(opponent.clone());
                    //     if add_graphics {
                    //         let projectile = Projectile::new(
                    //             Map2d::from_transform(transform),
                    //             Map2d::from_transform(opponent_pos),
                    //             3.0 * 2.0 * 2.0,
                    //         );
                    //         projectiles_entities.push(projectile);
                    //     };
                    // } else {
                    //     let unit_point = delta_pos.unit_point();
                    //     let dir: Map2d =
                    //         Direction::from_angle(unit_point.0 as f64, unit_point.1 as f64).into();
                    //     // transform.append_translation_xyz(dir.0, dir.1, 0.0);
                    // }
                }
                Retreat => {}
                MoveTo(p) => {
                    // println!("Distance Travelled: {}", unit.speed * delta);
                    let append_vector = p.move_by(unit.speed * delta);
                    if let Some(vector) = append_vector {
                        events.single_write(EffectEvents::Move(
                            entity,
                            Map2d::from(transform) + vector,
                        ));
                        unit.objective = Orders::AwaitingOrders;
                        // unit.mission = Orders::AwaitingOrders;
                    }
                    // events.single_write(EffectEvents::Move(entity, unit.speed * delta))
                    // if let Some(p) = p.move_by(unit.speed) {
                    //     transform.append_translation_xyz(p.0, p.1, 0.0);
                    // };
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
