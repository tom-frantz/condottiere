// use crate::components::unit::*;
// use amethyst::core::ecs::{Entities, Entity, Join, Read, ReadStorage, System, WriteStorage};
// use amethyst::core::Time;
// use std::ops::Deref;
// use std::time::Duration;
//
// type Damage = usize;
//
// pub enum Actions {
//     Damage {
//         current: Entity,
//         target: Entity,
//         damage: Damage,
//     },
// }
//
// #[derive(Default)]
// pub struct BattleSystem {
//     last_update: Duration,
// }
//
// impl<'s> System<'s> for BattleSystem {
//     type SystemData = (WriteStorage<'s, Unit>, Read<'s, Time>, Entities<'s>);
//
//     fn run(&mut self, (mut units, time, mut entities): Self::SystemData) {
//         self.last_update += time.delta_time();
//
//         if self.last_update.as_millis() > 1000 {
//             self.last_update -= Duration::new(1, 0);
//             println!("Update!")
//         } else {
//             return;
//         };
//
//         let mut actions: Vec<Actions> = Vec::new();
//
//         for (unit, entity) in (&mut units, &*entities).join() {
//             let action: Option<Actions> = match unit.objective {
//                 Objective::Attack(opponent) => Some(Actions::Damage {
//                     current: entity,
//                     target: opponent,
//                     damage: 2,
//                 }),
//                 Objective::Move((x, y)) => unimplemented!(),
//                 Objective::Hold => unimplemented!(),
//                 Objective::RequireNewOrder => unimplemented!(),
//             };
//
//             if let Some(action) = action {
//                 actions.push(action)
//             }
//         }
//
//         for action in actions {
//             match action {
//                 Actions::Damage {
//                     current,
//                     target,
//                     damage,
//                 } => {
//                     let target_unit = units.get_mut(target).unwrap();
//
//                     if let Some(new_hp) = target_unit.hp.checked_sub(damage) {
//                         target_unit.hp = new_hp;
//                         println!("{:?}", units.get(target).unwrap())
//                     } else {
//                         println!("Unit died! {:?}", target_unit);
//                         units.get_mut(current).unwrap().objective = Objective::RequireNewOrder;
//                         entities.delete(target);
//                     }
//                 }
//             };
//         }
//     }
// }
