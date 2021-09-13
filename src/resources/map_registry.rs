use amethyst::core::ecs::Entity;
use std::iter::Enumerate;

pub struct MapRegistry {
    rows: usize,
    columns: usize,
    tiles: Vec<Entity>,
}

impl MapRegistry {
    pub fn new(rows: usize, columns: usize, tiles: Vec<Entity>) -> Self {
        MapRegistry {
            rows,
            columns,
            tiles,
        }
    }

    pub fn add_tile(&mut self, tile: Entity) {
        self.tiles.push(tile)
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Entity> {
        self.tiles.get(x + y * self.columns)
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

struct MapRegistryIterator<'a> {
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

impl Iterator for MapRegistryIterator {
    type Item = ((usize, usize), Entity);

    fn next(&mut self) -> Option<Self::Item> {
        // Handle increment
        self.current.0 += 1;
        if self.current.0 == self.map_registry.columns {
            self.current.0 = 0;
            self.current.1 += 1;
            if self.current.1 == self.map_registry.rows {
                None
            }
        };

        // Send back correct next item
        let (x, y) = self.current;
        Some(((x, y), self.map_registry.get_tile(x, y).unwrap().clone()))
    }
}
