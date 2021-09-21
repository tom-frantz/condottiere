use crate::resources::*;
use amethyst::core::Transform;
use std::f32::consts::PI;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Map2d(pub f32, pub f32);

impl Map2d {
    pub fn magnitude(&self) -> f32 {
        (self.0).hypot(self.1)
    }

    pub fn unit_point(&self) -> Map2d {
        let mag = self.magnitude();
        Map2d(self.0 / mag, self.1 / mag)
    }

    pub fn by_speed(&self, speed: f32) -> Map2d {
        let mut unit = self.unit_point();
        unit.0 *= speed;
        unit.1 *= speed;
        unit
    }

    pub fn from_transform(transform: &Transform) -> Self {
        let translation = transform.translation();
        Map2d(translation.x, translation.y)
    }
}

impl PartialEq for Map2d {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Map2d {}

impl Add for Map2d {
    type Output = Map2d;

    fn add(self, rhs: Self) -> Self::Output {
        Map2d(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Map2d {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Map2d(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<Transform> for Map2d {
    type Output = Map2d;

    fn sub(self, rhs: Transform) -> Self::Output {
        let translation = rhs.translation();
        Map2d(self.0 - translation.x, self.1 - translation.y)
    }
}

pub fn get_real_location(x: usize, y: usize) -> (f32, f32) {
    (x as f32 * PIXEL_SIZE, y as f32 * PIXEL_SIZE)
}

pub fn move_at_speed(target: &Transform, source: &mut Transform, speed: f64) {
    let x = target.translation().x - source.translation().x;
    let y = target.translation().y - source.translation().y;

    let total = x.abs() + y.abs();

    let x_percent = ((x / total) * PI / 2.0).sin() * speed as f32; // [-1, 1] * speed
    let y_percent = ((y / total) * PI / 2.0).sin() * speed as f32;

    source.move_right(if x_percent.is_sign_positive() {
        x_percent.min(x)
    } else {
        x_percent.max(x)
    }); // +x
    source.move_up(if y_percent.is_sign_positive() {
        y_percent.min(y)
    } else {
        y_percent.max(y)
    }); // +y
}
