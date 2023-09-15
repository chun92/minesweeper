use bevy::prelude::*;
use super::grid::Grid;
use super::asset;

#[derive(Component)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    pub fn get_position(&self, grid: &Grid) -> Vec3 {
        assert!(self.x <= grid.width);
        assert!(self.y <= grid.height);
        assert!(self.x > 0);
        assert!(self.y > 0);

        let width_pixel = asset::texture_type::TextureAtlasType::Bombs.get_cell_size().0;
        let height_pixel = asset::texture_type::TextureAtlasType::Bombs.get_cell_size().1;
        let x_offset = grid.width as f32 * width_pixel / 2.0;
        let y_offset = grid.height as f32 * height_pixel / 2.0;
        let x = (self.x - 1) as f32 * width_pixel + width_pixel / 2.0 - x_offset;
        let y = (self.y - 1) as f32 * height_pixel + height_pixel / 2.0 - y_offset;
        Vec3::new(x, y, 0.0)
    }
}
