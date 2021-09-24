use crate::utils::movement::Map2d;
use amethyst::core::ecs::*;

pub mod order_creator;
pub mod order_executor;

#[derive(Clone, Debug)]
pub enum Orders {
    Attack(Entity),
    Retreat,
    MoveTo(Map2d),

    // Fixed location
    Hold,
    Ambush,
    DigIn,

    // Utility Orders
    AwaitingOrders,
}
