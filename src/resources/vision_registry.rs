use std::collections::{HashMap, HashSet};

use amethyst::core::ecs::ReadStorage;
use petgraph::graph::{NodeIndex, UnGraph};

use crate::components::terrain::Terrain;
use crate::resources::map_registry::MapRegistry;
use crate::resources::HeightPoint;

const _UNIT_HEIGHT: f32 = 1.0;

pub struct VisionRegistry {
    vision: UnGraph<(usize, usize), f32>, // node is a location, edge is the vision percentage [0, 1]
    graph_map: HashMap<(usize, usize), NodeIndex>,
    index_map: HashMap<NodeIndex, (usize, usize)>,
}

impl VisionRegistry {
    pub fn new(map_registry: &MapRegistry, _terrain: ReadStorage<Terrain>) -> Self {
        let mut vision: UnGraph<(usize, usize), f32> = UnGraph::new_undirected();
        let mut graph_map: HashMap<(usize, usize), NodeIndex> = HashMap::new();
        let mut index_map: HashMap<NodeIndex, (usize, usize)> = HashMap::new();

        for x in 0..(map_registry.columns) {
            for y in 0..(map_registry.rows) {
                let node_index = vision.add_node((x, y));
                graph_map.insert((x, y), node_index);
                index_map.insert(node_index, (x, y));
            }
        }

        {
            let get_vertex_lin_inter = Box::new(|x: f32, y: f32| {
                if x < 0.0
                    || y < 0.0
                    || x > map_registry.columns as f32
                    || y > map_registry.rows as f32
                {
                    return None;
                }

                let point: Option<f32> = if x.floor() != x {
                    let prev = map_registry
                        .get_vertex(x.floor() as usize, y as usize)
                        .unwrap();
                    let next = map_registry.get_vertex(x as usize, y as usize).unwrap();
                    Some(((next - prev) * (x % 1.0)) + prev)
                } else if y.floor() != y {
                    let prev = map_registry
                        .get_vertex(x as usize, y.floor() as usize)
                        .unwrap();
                    let next = map_registry.get_vertex(x as usize, y as usize).unwrap();
                    Some(((next - prev) * (y % 1.0)) + prev)
                } else {
                    map_registry
                        .get_vertex(x as usize, y as usize)
                        .map(|f| f.clone())
                };
                point
            });

            for ((self_x, self_y), (_self_entity, self_height)) in map_registry.iter() {
                for ((x, y), (_entity, height)) in map_registry.iter_from(self_x, self_y) {
                    let can_see = check_visibility(
                        (self_x as f32, self_y as f32, *self_height).into(),
                        (x as f32, y as f32, *height).into(),
                        |x, y| get_vertex_lin_inter(x, y),
                    );

                    if can_see {
                        vision.add_edge(
                            graph_map.get(&(x, y)).unwrap().clone(),
                            graph_map.get(&(self_x, self_y)).unwrap().clone(),
                            1.0,
                        );

                        vision.add_edge(
                            graph_map.get(&(self_x, self_y)).unwrap().clone(),
                            graph_map.get(&(x, y)).unwrap().clone(),
                            1.0,
                        );
                    }
                }
            }
        }

        VisionRegistry {
            vision,
            graph_map,
            index_map,
        }
    }

    pub fn _get_visible(&self, pos: (usize, usize)) -> Option<HashSet<(usize, usize)>> {
        let index = self.graph_map.get(&pos)?;

        let neighbours = self.vision.neighbors(index.clone());
        let vision_set: HashSet<(usize, usize)> = neighbours
            .map(|node_index| self.index_map.get(&node_index).unwrap().clone())
            .collect();

        Some(vision_set)
    }
}

fn get_next(val: f32, vec_diff: f32) -> f32 {
    if val % 1.0 != 0.0 {
        return if vec_diff.is_sign_positive() {
            val.ceil()
        } else {
            val.floor()
        };
    };

    return if vec_diff.is_sign_positive() {
        val + 1.0
    } else {
        val - 1.0
    };
}

fn check_visibility<F>(
    start_point: HeightPoint,
    end_point: HeightPoint,
    get_vertex_lin_inter: F,
) -> bool
where
    F: Fn(f32, f32) -> Option<f32>,
{
    let difference: HeightPoint = end_point - start_point;
    let magnitude: f32 = difference.x.abs() + difference.y.abs();
    let _difference_hypot = difference.x.hypot(difference.y);

    let dx = difference.x / magnitude;
    let dy = difference.y / magnitude;
    let d_height = difference.height / magnitude;

    let mut current_point = start_point.clone();

    let mut count = 0;

    while current_point != end_point && count < 100 {
        count += 1;

        // Calculate the next point that intersects with a boundary
        let next_point: HeightPoint = (
            get_next(current_point.x, difference.x),
            get_next(current_point.y, difference.y),
        )
            .into();

        let mut unit_point: HeightPoint = (
            (next_point.x - current_point.x) / dx,
            (next_point.y - current_point.y) / dy,
        )
            .into();

        if unit_point.x.abs() < unit_point.y.abs() {
            unit_point.y = unit_point.x;
            unit_point.height = unit_point.x;
        } else {
            unit_point.x = unit_point.y;
            unit_point.height = unit_point.y;
        }

        let distance_point: HeightPoint = (
            unit_point.x * dx,
            unit_point.y * dy,
            unit_point.height * d_height,
        )
            .into();

        let _distance_travelled = current_point - start_point;

        current_point = current_point + distance_point;

        // Find the height
        let actual_height = get_vertex_lin_inter(current_point.x, current_point.y);
        if actual_height.unwrap() > (current_point.height + 1.0) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::resources::vision_registry::*;

    #[test]
    fn trial() {
        let start_point: HeightPoint = (4.0, 4.0, 8.0).into();
        let end_point: HeightPoint = (4.0, 2.0, 6.0).into();

        let is_visible = check_visibility(start_point, end_point, |x, y| Some(x + y));
    }
}
