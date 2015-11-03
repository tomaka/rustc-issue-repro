use std::mem;

use cgmath::Vector2;
use image;

use resources;

pub fn load(mod_name: Option<&str>, tile_dimensions: &Vector2<f32>) -> Map {
    Map {
        tiles: Vec::new(),
        num_tiles_x: 0,
        num_tiles_y: 0,
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub num_tiles_x: usize,
    pub num_tiles_y: usize,
}

#[derive(Debug, Clone)]
pub struct Tile {
    /// Number of tiles to the left of this one on the world map (includes hypothetical water tiles)
    pub x_index: u32,
    /// Number of tiles to the bottom of this one on the world map (includes hypothetical water tiles)
    pub y_index: u32,
    pub bottom_tile_index: Option<usize>,
    pub top_tile_index: Option<usize>,
    pub left_tile_index: Option<usize>,
    pub right_tile_index: Option<usize>,

    /// RGB color (between 0 and 255) of the color of the nation whose tile belongs to.
    pub owner_nation_color: [u8; 3],
}
