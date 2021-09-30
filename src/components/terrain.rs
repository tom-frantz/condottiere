use amethyst::core::ecs::{Component, DenseVecStorage};

pub const TILE_REAL_SIZE: f32 = 100.0; // The tile's real world equivalent size.

#[derive(Default)]
pub struct Terrain {
    pub height: f32,
}

impl Terrain {
    pub fn new(height: f32) -> Terrain {
        Terrain {
            height,
            ..Terrain::default()
        }
    }
}

impl Component for Terrain {
    type Storage = DenseVecStorage<Self>;
}
