use bevy::prelude::*;
use super::grid::Grid;
use super::asset;

#[derive(Component)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub is_mine: bool,
    pub num_mines_around: u32,
    pub is_revealed: bool,
    pub is_flagged: bool,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_both_pressed: bool,
}

impl Cell {
    pub fn new(x: u32, y: u32, is_mine: bool, num_mines_around: u32) -> Self {
        Self { x, 
            y, 
            is_mine, 
            num_mines_around, 
            is_revealed: false,
            is_flagged: false,
            is_left_pressed: false,
            is_right_pressed: false,
            is_both_pressed: false,
        }
    }

    pub fn get_position(&self, grid: &Grid) -> Vec3 {
        assert!(self.x <= grid.width);
        assert!(self.y <= grid.height);
        assert!(self.x > 0);
        assert!(self.y > 0);

        let width_pixel = asset::texture_type::TextureAtlasType::Cells.get_cell_size().0;
        let height_pixel = asset::texture_type::TextureAtlasType::Cells.get_cell_size().1;
        let x_offset = grid.width as f32 * width_pixel / 2.0;
        let y_offset = grid.height as f32 * height_pixel / 2.0;
        let x = (self.x - 1) as f32 * width_pixel + width_pixel / 2.0 - x_offset;
        let y = (self.y - 1) as f32 * height_pixel + height_pixel / 2.0 - y_offset;
        Vec3::new(x, y, 0.0)
    }

    pub fn left_pressed(&mut self) {
        self.is_left_pressed = true;
        info!("left pressed: ({}, {})", self.x, self.y);
    }

    pub fn left_out(&mut self) {
        self.is_left_pressed = false;
        info!("left out: ({}, {})", self.x, self.y);
    }

    pub fn left_released(&mut self) {
        self.is_left_pressed = false;
        info!("left released: ({}, {})", self.x, self.y);
    }

    pub fn right_pressed(&mut self) {
        self.is_right_pressed = true;
        info!("right pressed: ({}, {})", self.x, self.y);
    }

    pub fn right_out(&mut self) {
        self.is_right_pressed = false;
        info!("right out: ({}, {})", self.x, self.y);
    }

    pub fn right_released(&mut self) {
        self.is_right_pressed = false;
        info!("right released: ({}, {})", self.x, self.y);
    }

    pub fn both_pressed(&mut self) {
        self.is_both_pressed = true;
        info!("both pressed: ({}, {})", self.x, self.y);
    }

    pub fn both_out(&mut self) {
        self.is_both_pressed = false;
        info!("both out: ({}, {})", self.x, self.y);
    }

    pub fn both_released(&mut self) {
        self.is_both_pressed = false;
        info!("both released: ({}, {})", self.x, self.y);
    }
}
