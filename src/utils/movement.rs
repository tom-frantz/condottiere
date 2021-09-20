use crate::resources::*;
use amethyst::core::Transform;
use std::f32::consts::PI;

pub type MapPoint = (usize, usize);

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
