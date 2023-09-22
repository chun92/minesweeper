use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::seq::SliceRandom;

use super::asset::texture_type::TextureType;
#[derive(Resource)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub mine_positions: HashSet<(u32, u32)>,
    pub window_position: Vec2,
    pub window_size: Vec2,
    pub cells: Vec<(u32, u32, Entity)>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: 30,
            height: 16,
            mine_positions: HashSet::new(),
            window_position: Vec2::new(0.0, 0.0),
            window_size: Vec2::new(0.0, 0.0),
            cells: Vec::new(),
        }
    }
}

impl Grid {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn init(
        &mut self,
        width: u32,
        height: u32,
        window_width: f32,
        window_height: f32,
    ) {
        self.width = width;
        self.height = height;
        
        let cell_width = TextureType::Cells.get_cell_size().0;
        let cell_height = TextureType::Cells.get_cell_size().1;
    
        let grid_width = cell_width * self.width as f32;
        let grid_height = cell_height * self.height as f32;
    
        self.window_position = Vec2::new(window_width / 2.0, window_height / 2.0);
        self.window_size = Vec2::new(grid_width, grid_height);

        self.cells.clear();
    }

    pub fn create_mine_positions(&mut self, num_mines: u32, exclude_pos: Option<(u32, u32)>) {
        let mut rng = rand::thread_rng();
        
        let num_mines = if num_mines > self.width * self.height {
            self.width * self.height
        } else {
            num_mines
        };
    
        // Create a vector with all possible positions
        let mut positions: Vec<(u32, u32)> = (1..=self.width)
            .flat_map(|x| (1..=self.height).map(move |y| (x, y)))
            .collect();

        if let Some ((x, y)) = exclude_pos {
            positions.retain(|(x_, y_)| *x_ != x || *y_ != y);
        }
    
        positions.shuffle(&mut rng);
    
        self.mine_positions = positions.into_iter().take(num_mines as usize).collect::<HashSet<_>>()
    }

    pub fn add_cell(&mut self, x: u32, y: u32, entity: Entity) {
        self.cells.push((x, y, entity));
    }

    pub fn find_cell(&self, x: u32, y: u32) -> Option<Entity> {
        for (x_, y_, entity) in &self.cells {
            if *x_ == x && *y_ == y {
                return Some(*entity);
            }
        }
        None
    }

    pub fn get_arround_cells(&self, x: u32, y: u32) -> Vec<(u32, u32, Option<Entity>)> {
        let width = self.width;
        let height = self.height;

        let mut cells = Vec::new();
        // 1 2 3
        // 4 x 5
        // 6 7 8

        // case 1
        if x > 1 && y > 1 {
            cells.push((x - 1, y - 1, self.find_cell(x - 1, y - 1)));
        }
        // case 2
        if y > 1 {
            cells.push((x, y - 1, self.find_cell(x, y - 1)));
        }
        // case 3
        if x < width && y > 1 {
            cells.push((x + 1, y - 1, self.find_cell(x + 1, y - 1)));
        }
        // case 4
        if x > 1 {
            cells.push((x - 1, y, self.find_cell(x - 1, y)));
        }
        // case 5
        if x < width {
            cells.push((x + 1, y, self.find_cell(x + 1, y)));
        }
        // case 6
        if x > 1 && y < height {
            cells.push((x - 1, y + 1, self.find_cell(x - 1, y + 1)));
        }
        // case 7
        if y < height {
            cells.push((x, y + 1, self.find_cell(x, y + 1)));
        }
        // case 8
        if x < width && y < height {
            cells.push((x + 1, y + 1, self.find_cell(x + 1, y + 1)));
        }

        cells
    }

    pub fn get_num_mines_around(&self, x: u32, y: u32) -> u32 {
        let mine_positions = &self.mine_positions;
        let mut num_mines_around = 0;
        let arround_cells = self.get_arround_cells(x, y);
        for (x, y, _) in arround_cells {
            if mine_positions.contains(&(x, y)) {
                num_mines_around += 1;
            }
        }
        num_mines_around
    }
}
