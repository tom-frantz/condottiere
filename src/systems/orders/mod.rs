use amethyst::core::ecs::*;

pub mod order_creator;
pub mod order_executor;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub enum Orders {
    Attack(Entity),
    Retreat,
    MoveTo((usize, usize)),

    // Fixed location
    Hold,
    Ambush,
    DigIn,

    // Utility Orders
    AwaitingOrders,
}
