use crate::systems::orders::Orders;
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

#[derive(Debug, Clone)]
pub struct Unit {
    pub order: Orders,
    pub hp: usize,
    pub engagement_distance: f32,
}

impl Unit {
    pub fn new(hp: usize, order: Orders) -> Self {
        Unit {
            hp,
            order,
            engagement_distance: 100.0,
        }
    }

    pub fn build_entity() -> UnitBuilder {
        UnitBuilder::default()
    }

    pub fn set_objective(&mut self, order: Orders) {
        self.order = order;
    }
}

#[derive(Default)]
pub struct UnitBuilder {
    hp: usize,
    pos: (f32, f32),
    colour: Tint,
    sprite: Option<SpriteRender>,
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

    pub fn hp(mut self, hp: usize) -> Self {
        self.hp = hp;
        self
    }

    pub fn sprite(mut self, sprite: SpriteRender) -> Self {
        self.sprite = Some(sprite);
        self
    }

    pub fn create(mut self, world: &mut World) -> Entity {
        let mut transform = Transform::default();
        transform.set_translation_xyz(self.pos.0, self.pos.1, 0.0);

        let hp = if self.hp == 0 { 20 } else { self.hp };

        let mut entity = world
            .create_entity()
            .with(transform)
            .with(self.colour)
            .with(self.sprite.unwrap())
            .with(Unit::new(hp, Orders::AwaitingOrders));

        entity.build()
    }
}

impl Component for Unit {
    type Storage = DenseVecStorage<Self>;
}