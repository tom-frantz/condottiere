use crate::components::terrain::Terrain;
use crate::resources::*;
use amethyst::core::ecs::Entity;
use amethyst::core::Transform;
use amethyst::prelude::*;
use amethyst::renderer::palette::rgb::Rgb;
use amethyst::renderer::resources::Tint;
use amethyst::renderer::SpriteRender;
use std::any::Any;
use std::iter::Enumerate;

pub struct MapRegistry {
    pub rows: usize,
    pub columns: usize,
    tiles: Vec<(Entity, f32)>,
    vertex_heights: Vec<f32>,
}

impl MapRegistry {
    pub fn new(rows: usize, columns: usize, world: &mut World, sprite: SpriteRender) -> Self {
        let mut tiles: Vec<(Entity, f32)> = Self::generate_map(rows, columns, world, sprite);
        let mut vertex_heights: Vec<f32> = Vec::with_capacity((rows + 1) * (columns + 1));

        for y in 0..=rows {
            for x in 0..=columns {
                let vertex_height = Self::generate_vertex_height(&tiles, columns, x, y);
                vertex_heights.push(vertex_height)
            }
        }

        MapRegistry {
            rows,
            columns,
            tiles,
            vertex_heights,
        }
    }

    fn generate_map(
        rows: usize,
        columns: usize,
        world: &mut World,
        sprite: SpriteRender,
    ) -> Vec<(Entity, f32)> {
        let mut tiles: Vec<(Entity, f32)> = Vec::new();

        for y in 0..rows {
            for x in 0..columns {
                let mut transform = Transform::default();
                transform.set_translation_xyz(
                    (x as f32 * PIXEL_SIZE),
                    (y as f32 * PIXEL_SIZE),
                    -1.0,
                );

                let mut tint = Tint::default();
                // let height = -((x as isize - 5) as f32 * 5.0 as f32).abs() + 25.0;
                let height: f32 = x as f32 + y as f32;
                tint.0.color = Rgb::new(0.0, height / 25.0, 0.0);

                let tile = world
                    .create_entity()
                    .with(transform)
                    .with(sprite.clone())
                    .with(tint)
                    .with(Terrain::new(height))
                    .build();

                tiles.push((tile, height));
            }
        }

        tiles
    }

    fn generate_vertex_height(
        items: &Vec<(Entity, f32)>,
        columns: usize,
        x: usize,
        y: usize,
    ) -> f32 {
        let get_height = |x: isize, y: isize| {
            if x < 0 || y < 0 || x >= columns as isize || y >= columns as isize {
                return None;
            }

            items
                .get((x as usize + y as usize * (columns) as usize) as usize)
                .map(|item| item.1)
        };

        // tr, br, bl, tl (clockwise from 12 o'clock)
        let neighbours_source = [
            get_height(x as isize, y as isize),
            get_height(x as isize, y as isize - 1),
            get_height(x as isize - 1, y as isize - 1),
            get_height(x as isize - 1, y as isize),
        ];

        let amount = neighbours_source
            .iter()
            .skip_while(|item| (**item).is_none())
            .clone()
            .count();

        println!("{} {} {:?}", x, y, neighbours_source);

        neighbours_source
            .iter()
            .filter(|item| (**item).is_some())
            .map(|item| item.unwrap())
            .sum::<f32>()
            / amount as f32
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&(Entity, f32)> {
        if x < 0 || y < 0 || x >= self.columns || y >= self.rows {
            return None;
        }

        self.tiles.get(x + y * self.columns)
    }

    pub fn get_vertex(&self, x: usize, y: usize) -> Option<&f32> {
        if x < 0 || y < 0 || x > self.columns || y > self.rows {
            return None;
        }

        self.vertex_heights.get(x + y * (self.columns + 1))
    }

    pub fn iter(&self) -> MapRegistryIterator {
        MapRegistryIterator::new(&self)
    }

    pub fn iter_from(&self, x: usize, y: usize) -> MapRegistryIterator {
        MapRegistryIterator {
            map_registry: self,
            current: (x, y),
        }
    }
}

pub struct MapRegistryIterator<'a> {
    map_registry: &'a MapRegistry,
    current: (usize, usize),
}

impl<'a> MapRegistryIterator<'a> {
    pub fn new(map_registry: &'a MapRegistry) -> MapRegistryIterator {
        MapRegistryIterator {
            map_registry,
            current: (0, 0),
        }
    }
}

impl Iterator for MapRegistryIterator<'_> {
    type Item = ((usize, usize), (Entity, f32));

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current;

        if self.current.1 == self.map_registry.rows {
            return None;
        }

        // Handle increment
        self.current.0 += 1;
        if self.current.0 == self.map_registry.columns {
            self.current.0 = 0;
            self.current.1 += 1;
        };

        // Send back correct next item
        let (x, y) = current;
        Some(((x, y), self.map_registry.get_tile(x, y).unwrap().clone()))
    }
}
