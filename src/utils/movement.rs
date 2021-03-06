use self::Direction::{Down, DownLeft, DownRight, Left, Right, Up, UpLeft, UpRight};
use crate::resources::*;
use amethyst::core::num::Pow;
use amethyst::core::Transform;
use std::f32::consts::PI;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    pub fn from_angle(x: f64, y: f64) -> Self {
        let angle_bracket = 45.0;
        let tolerance = angle_bracket / 2.0;

        let degrees = x.atan2(y) * 180.0 / std::f64::consts::PI;
        let direction = (((degrees + tolerance).rem_euclid(360.0)) / angle_bracket).floor();
        println!("x {}, y {}, dir {}", x, y, direction);
        match direction as usize {
            0 => Up,
            1 => UpRight,
            2 => Right,
            3 => DownRight,
            4 => Down,
            5 => DownLeft,
            6 => Left,
            7 => UpLeft,
            _ => panic!("Error getting direction from vector ({}, {})", x, y),
        }
    }
}

impl From<Direction> for Map2d {
    fn from(dir: Direction) -> Self {
        match dir {
            Up => Map2d(0.0, 1.0),
            UpRight => Map2d(1.0, 1.0),
            Right => Map2d(1.0, 0.0),
            DownRight => Map2d(1.0, -1.0),
            Down => Map2d(0.0, -1.0),
            DownLeft => Map2d(-1.0, -1.0),
            Left => Map2d(-1.0, 0.0),
            UpLeft => Map2d(-1.0, 1.0),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Map2d(pub f32, pub f32);

impl Map2d {
    pub fn magnitude(&self) -> f32 {
        (self.0).hypot(self.1).abs()
    }

    pub fn unit_point(&self) -> Map2d {
        let mag = self.magnitude();
        Map2d(
            (self.0 / mag).pow(2) * self.0.signum(),
            (self.1 / mag).pow(2) * self.1.signum(),
        )
    }

    pub fn by_speed(&self, speed: f32) -> Map2d {
        let mut unit = self.unit_point();
        unit.0 *= speed;
        unit.1 *= speed;
        unit
    }

    pub fn from_transform(transform: &Transform) -> Self {
        let translation = transform.translation();

        Map2d(translation.x / PIXEL_SIZE, translation.y / PIXEL_SIZE)
    }

    pub fn to_transform(&self, z: f32) -> Transform {
        let mut transform = Transform::default();
        transform.set_translation_xyz(self.0 * PIXEL_SIZE, self.1 * PIXEL_SIZE, z);
        transform
    }
}

impl From<&mut Transform> for Map2d {
    fn from(transform: &mut Transform) -> Self {
        let translation = transform.translation();
        Map2d(translation.x / PIXEL_SIZE, translation.y / PIXEL_SIZE)
    }
}

impl From<&Transform> for Map2d {
    fn from(transform: &Transform) -> Self {
        let translation = transform.translation();
        Map2d(translation.x / PIXEL_SIZE, translation.y / PIXEL_SIZE)
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

pub fn _move_at_speed(target: &Transform, source: &mut Transform, speed: f64) {
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

#[cfg(test)]
mod test {
    use crate::utils::movement::Map2d;

    #[test]
    fn magnitudes() {
        assert_eq!(Map2d(1.0, 1.0).magnitude(), 2_f32.sqrt());
        assert_eq!(Map2d(-1.0, 1.0).magnitude(), 2_f32.sqrt());
        assert_eq!(Map2d(1.0, -1.0).magnitude(), 2_f32.sqrt());
        assert_eq!(Map2d(-1.0, -1.0).magnitude(), 2_f32.sqrt());
    }

    #[test]
    fn unit() {
        assert_eq!(Map2d(1.0, 1.0).unit_point(), Map2d(0.49999997, 0.49999997));
        assert_eq!(
            Map2d(-1.0, 1.0).unit_point(),
            Map2d(-0.49999997, 0.49999997)
        );
        assert_eq!(
            Map2d(1.0, -1.0).unit_point(),
            Map2d(0.49999997, -0.49999997)
        );
        assert_eq!(
            Map2d(-1.0, -1.0).unit_point(),
            Map2d(-0.49999997, -0.49999997)
        );
    }

    #[test]
    fn by_speed() {
        assert_eq!(
            Map2d(-1.0, 1.0).by_speed(2.0),
            Map2d(-0.99999994, 0.99999994)
        );
    }
}
