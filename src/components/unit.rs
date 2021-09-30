use crate::components::equipment::Equipment;
use crate::components::unit::DamageResult::{Alive, Dead};
use crate::systems::orders::Orders;
use crate::utils::camera::CameraHeight;
use crate::utils::movement::get_real_location;
use amethyst::core::ecs::{Component, DenseVecStorage, Entity, World};
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::palette::rgb::Rgb;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::SpriteRender;

pub enum Team {
    Player,
    Enemy,
}

pub enum DamageResult {
    Alive,
    Dead,
}

#[derive(Debug, Clone)]
pub struct Unit {
    // What they've been assigned to do as a higher level objective.
    pub mission: Orders,
    // What they're currently doing to achieve their order.
    pub objective: Orders,
    pub hp: f32,
    pub engagement_distance: f32,
    pub speed: f32,
}

impl Unit {
    pub fn new(hp: f32, goal: Orders) -> Self {
        Unit {
            hp,
            mission: goal,
            engagement_distance: 100.0,
            objective: Orders::AwaitingOrders,
            speed: 50.0,
        }
    }

    pub fn build_entity() -> UnitBuilder {
        UnitBuilder::default()
    }

    pub fn set_objective(&mut self, order: Orders) {
        self.mission = order;
    }

    pub fn take_damage(&mut self, damage: f32) -> DamageResult {
        self.hp -= damage;

        if self.hp <= 0.0 {
            Dead
        } else {
            Alive
        }
    }
}

#[derive(Default)]
pub struct UnitBuilder {
    hp: f32,
    pos: (f32, f32),
    colour: Tint,
    sprite: Option<SpriteRender>,
    equipment: Option<Equipment>,
}

impl UnitBuilder {
    pub fn pos(mut self, x: usize, y: usize) -> Self {
        self.pos = get_real_location(x, y);
        self
    }

    pub fn team(mut self, team: Team) -> Self {
        self.colour.0.color = match team {
            Team::Player => Rgb::new(0.0, 0.0, 1.0),
            Team::Enemy => Rgb::new(1.0, 0.0, 0.0),
        };
        self
    }

    pub fn hp(mut self, hp: f32) -> Self {
        self.hp = hp;
        self
    }

    pub fn sprite(mut self, sprite: SpriteRender) -> Self {
        self.sprite = Some(sprite);
        self
    }

    pub fn equipment(mut self, equipment: Equipment) -> Self {
        self.equipment = Some(equipment);
        self
    }

    pub fn create(self, world: &mut World) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_xyz(self.pos.0, self.pos.1, CameraHeight::Units as u8 as f32);

        let hp: f32 = if self.hp == 0.0 { 20.0 } else { self.hp };

        let mut entity = world
            .create_entity()
            .with(transform)
            .with(self.colour)
            .with(self.sprite.unwrap())
            .with(Unit::new(hp, Orders::AwaitingOrders));

        if self.equipment.is_some() {
            entity = entity.with(self.equipment.unwrap());
        }

        entity.build()
    }
}

impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}
