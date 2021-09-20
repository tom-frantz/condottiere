use crate::utils::movement::MapPoint;
use amethyst::core::ecs::{Component, DenseVecStorage};

pub struct Projectile {
    start: MapPoint,
    end: MapPoint,
    speed: f32,
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
