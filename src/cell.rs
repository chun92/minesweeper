use bevy::prelude::*;
use super::grid::Grid;
use super::asset;

pub enum CellState {
    Hidden,
    Pressed,
    Revealed,
    Flagged,
    WrongFlagged,
    Exploded,
}

#[derive(Component, Default)]
pub struct Cells();

impl Cells {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Component)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub is_mine: bool,
    pub num_mines_around: u32,
    pub state: CellState,
    pub is_left_pressed: bool,
    pub is_right_pressed: bool,
    pub is_both_pressed: bool,
    pub is_opening: bool,
}

impl Cell {
    pub fn new(x: u32, y: u32, is_mine: bool, num_mines_around: u32) -> Self {
        Self { x, 
            y, 
            is_mine, 
            num_mines_around, 
            state: CellState::Hidden,
            is_left_pressed: false,
            is_right_pressed: false,
            is_both_pressed: false,
            is_opening: false,
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
        if self.is_left_pressed {
            return;
        }
        self.is_left_pressed = true;
        
        match self.state {
            CellState::Hidden => {
                self.state = CellState::Pressed;
            },
            _ => {}
        }
    }

    pub fn left_out(&mut self) {
        if !self.is_left_pressed {
            return;
        }
        self.is_left_pressed = false;

        match self.state {
            CellState::Hidden => {
                panic!("left out should not be called when cell is hidden");
            }
            CellState::Pressed => {
                self.state = CellState::Hidden;
            },
            _ => {}
        }
    }

    pub fn left_released(&mut self) {
        if !self.is_left_pressed {
            return;
        }
        self.is_left_pressed = false;

        match self.state {
            CellState::Hidden => {
                panic!("left released should not be called when cell is hidden");
            },
            CellState::Pressed => {
                self.is_opening = true;
            },
            _ => {}
        }
    }

    pub fn right_pressed(&mut self) {
        if self.is_right_pressed {
            return;
        }
        self.is_right_pressed = true;
        
        match self.state {
            CellState::Hidden => {
                self.state = CellState::Flagged;
            },
            CellState::Flagged => {
                self.state = CellState::Hidden;
            },
            _ => {}
        }
    }

    pub fn right_out(&mut self) {
        if !self.is_right_pressed {
            return;
        }
        self.is_right_pressed = false;
    }

    pub fn right_released(&mut self) {
        if !self.is_right_pressed {
            return;
        }
        self.is_right_pressed = false;
    }

    pub fn both_pressed(&mut self) {
        if self.is_both_pressed {
            return;
        }
        self.is_both_pressed = true;
        match self.state {
            CellState::Revealed => {
                // TODO:
            },
            _ => {}
        }
    }

    pub fn both_out(&mut self) {
        if !self.is_both_pressed {
            return;
        }
        self.is_both_pressed = false;
        match self.state {
            CellState::Revealed => {
                // TODO:
            },
            _ => {}
        }
    }

    pub fn both_released(&mut self) {
        if !self.is_both_pressed {
            return;
        }
        self.is_both_pressed = false;
        match self.state {
            CellState::Revealed => {
                // TODO:
            },
            _ => {}
        }
    }

    pub fn get_texture_index(&self) -> u32 {
        match self.state {
            CellState::Hidden => 0,
            CellState::Pressed => 1,
            CellState::Revealed => {
                if self.is_mine {
                    5
                } else if self.num_mines_around == 0 {
                    1 
                } else {
                    self.num_mines_around + 7
                }
            },
            CellState::Flagged => 2,
            CellState::WrongFlagged => 7,
            CellState::Exploded => 6,
        }
    }

    pub fn open(&mut self) -> bool {
        self.state = CellState::Revealed;
        !self.is_mine
    }
}
