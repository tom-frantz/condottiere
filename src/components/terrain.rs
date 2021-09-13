use amethyst::core::ecs::storage::UnprotectedStorage;
use amethyst::core::ecs::{Component, DenseVecStorage};
use std::any::Any;

pub const TILE_REAL_SIZE: f32 = 100.0; // The tile's real world equivalent size.

#[derive(Default)]
pub struct Terrain {}

impl Terrain {
    pub fn new() -> Terrain {
        Terrain {}
    }
}

impl Component for Terrain {
    type Storage = DenseVecStorage<Self>;
}
