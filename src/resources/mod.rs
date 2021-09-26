use std::ops::{Add, Sub};

pub const PIXEL_SIZE: f32 = 2.0;
pub const GRID_SIZE: usize = 10;

pub const BUFFER: f32 = PIXEL_SIZE * 3.0;
pub const CAMERA_DIMENSIONS: f32 = GRID_SIZE as f32 * PIXEL_SIZE + BUFFER;
pub const PIXEL_OFFSET: f32 = BUFFER / PIXEL_SIZE + (PIXEL_SIZE / 2.0);

use std::f32;

pub mod map_generators;
pub mod map_registry;
pub mod sprites_registry;
pub mod vision_registry;

#[derive(Clone, Debug, Copy)]
struct HeightPoint {
    pub x: f32,
    pub y: f32,
    pub height: f32,
}

impl Add for HeightPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        HeightPoint {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            height: self.height + rhs.height,
        }
    }
}

impl Sub for HeightPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        HeightPoint {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            height: self.height - rhs.height,
        }
    }
}

impl PartialEq<Self> for HeightPoint {
    fn eq(&self, other: &Self) -> bool {
        ulps_eq!(self.x, other.x, epsilon = 0.0001)
            && ulps_eq!(self.y, other.y, epsilon = 0.0001)
            && ulps_eq!(self.height, other.height, epsilon = 0.0001)
    }
}

impl Eq for HeightPoint {}

impl From<(f32, f32)> for HeightPoint {
    fn from(point: (f32, f32)) -> Self {
        HeightPoint {
            x: point.0,
            y: point.1,
            height: 0.0,
        }
    }
}

impl From<(f32, f32, f32)> for HeightPoint {
    fn from((x, y, z): (f32, f32, f32)) -> Self {
        HeightPoint { x, y, height: z }
    }
}
