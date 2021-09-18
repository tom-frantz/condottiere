use crate::components::terrain::Terrain;

pub mod linear_map;

pub trait MapGenerator {
    fn generate_map(columns: usize, rows: usize) -> Vec<Terrain>;
}
