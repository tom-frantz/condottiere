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

    pub fn next(&mut self, time_delta: f32, current_point: Map2d) -> Option<Map2d> {
        let unit = self.speed * time_delta;
        let next = (current_point + self.vector.by_speed(unit));

        if (self.end - current_point).magnitude() < (self.end - next).magnitude() {
            None
        } else {
            Some(next)
        }
    }
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}
