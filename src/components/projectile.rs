use crate::utils::movement::Map2d;
use amethyst::core::ecs::{Component, DenseVecStorage};

#[derive(Debug, Clone)]
pub struct Projectile {
    pub start: Map2d,
    end: Map2d,

    vector: Map2d,

    speed: f32,
}

impl Projectile {
    pub fn new(start: Map2d, end: Map2d, speed: f32) -> Self {
        let vector: Map2d = end - start;
        Projectile {
            start,
            end,
            speed,
            vector,
        }
    }

    pub fn next(&self, time_delta: f32, current_point: Map2d) -> Option<Map2d> {
        if self.end == current_point {
            return None;
        }

        let unit = self.speed * time_delta;

        if (self.end - current_point).magnitude() <= unit {
            return Some(self.end);
        }

        Some(current_point + self.vector.by_speed(unit))
    }
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
