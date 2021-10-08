use amethyst::core::ecs::Entity;

pub struct CameraEntity {
    pub entity: Entity,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl CameraEntity {
    pub fn new(entity: Entity, width: f32, height: f32) -> Self {
        CameraEntity {
            x: 0.0,
            y: 0.0,
            entity,
            width,
            height,
        }
    }
}
