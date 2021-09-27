use crate::components::terrain::TILE_REAL_SIZE;
use crate::utils::movement::Map2d;
use amethyst::core::ecs::*;

pub mod effects_system;
pub mod order_creator;
pub mod order_executor;

#[derive(Clone, Debug)]
pub enum Orders {
    Attack(Entity),
    Retreat,
    MoveTo(MoveToOrder),

    // Fixed location
    Hold,
    Ambush,
    DigIn,

    // Utility Orders
    AwaitingOrders,
}

#[derive(Clone, Debug)]
pub struct MoveToOrder {
    to: Map2d,
    vector: Map2d,
    distance_travelled: f32,
    total_distance: f32,
}

impl MoveToOrder {
    pub fn new(current: Map2d, to: Map2d) -> Self {
        let vector = to - current;

        println!(
            "vector.magnitude() {} * TILE_REAL_SIZE = {}",
            vector.magnitude(),
            vector.magnitude() * TILE_REAL_SIZE
        );

        MoveToOrder {
            to,
            vector,
            distance_travelled: 0.0,
            total_distance: vector.magnitude() * TILE_REAL_SIZE,
        }
    }

    pub fn move_by(&mut self, distance: f32) -> Option<Map2d> {
        self.distance_travelled += distance;
        // println!("Distance {} total: {}", distance, self.distance_travelled);
        if self.distance_travelled >= self.total_distance {
            Some(self.vector.clone())
        } else {
            None
        }
    }
}
