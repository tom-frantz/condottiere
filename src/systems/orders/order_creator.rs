use crate::components::equipment::Equipment;
use crate::components::projectile::Projectile;
use crate::components::terrain::{Terrain, TILE_REAL_SIZE};
use crate::components::unit::Unit;
use crate::resources::sprites_registry::SpriteRegistry;
use crate::systems::orders::Orders::*;
use crate::systems::orders::{MoveToOrder, Orders};
use crate::systems::rendering::new_renders::RenderEvents;
use crate::utils::movement::{Direction, Map2d};
use amethyst::core::ecs::*;
use amethyst::core::{Time, Transform};
use amethyst::prelude::*;
use amethyst::renderer::SpriteRender;
use amethyst::{core::SystemDesc, derive::SystemDesc};

#[derive(SystemDesc)]
#[system_desc(name(OrderCreatorSystemDesc))]
pub struct OrderCreatorSystem {
    #[system_desc(event_channel_reader)]
    render_system_event_id: ReaderId<RenderEvents>,
}

impl OrderCreatorSystem {
    fn new(render_system_event_id: ReaderId<RenderEvents>) -> Self {
        OrderCreatorSystem {
            render_system_event_id,
        }
    }
}

impl<'s> System<'s> for OrderCreatorSystem {
    type SystemData = (
        WriteStorage<'s, Unit>,
        ReadStorage<'s, Equipment>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Projectile>,
        ReadStorage<'s, SpriteRender>,
        ReadExpect<'s, SpriteRegistry>,
        ReadStorage<'s, Terrain>,
        Entities<'s>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (
            mut units,
            equipment_components,
            transforms,
            projectiles,
            sprites,
            sprite_registry,
            terrain,
            entities,
            time,
        ): Self::SystemData,
    ) {
        for (unit, equipment, transform, entity) in
            (&mut units, &equipment_components, &transforms, &*entities).join()
        {
            let next_order: Option<Orders> = match &unit.mission {
                Attack(opponent) => {
                    let opponent_pos = transforms.get(opponent.clone()).unwrap();
                    let delta_pos = (Map2d::from(opponent_pos) - Map2d::from(transform));

                    let range = equipment.equipment.stats().range;

                    // Check if the unit is in range, then set objectives
                    if delta_pos.magnitude() * TILE_REAL_SIZE <= range {
                        unit.objective = Orders::Attack(opponent.clone());
                        None
                    } else {
                        if let MoveTo(_) = unit.objective {
                            None
                        } else {
                            // If not in range, change the objective to move close enough to engage.
                            let unit_point = delta_pos.unit_point();
                            let dir =
                                Direction::from_angle(unit_point.0 as f64, unit_point.1 as f64);
                            println!("DIR {:?} {:?} {:?}", Map2d::from(transform), delta_pos, dir);
                            Some(MoveTo(MoveToOrder::new(
                                transform.into(),
                                Map2d::from(dir) + transform.into(),
                            )))
                        }
                    }
                }
                Retreat => None,
                MoveTo(p) => None,
                Hold => None,
                Ambush => None,
                DigIn => None,
                AwaitingOrders => None,
            };

            if let Some(new_order) = next_order {
                unit.objective = new_order;
            }
        }
    }
}
