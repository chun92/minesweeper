use bevy::prelude::*;
use rand::seq::SliceRandom;
use bevy::utils::HashSet;
#[derive(Resource)]
pub struct Grid {
    pub width: u32,
    pub height: u32,
    pub mine_positions: HashSet<(u32, u32)>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: 30,
            height: 16,
            mine_positions: HashSet::new(),
        }
    }
}

impl Grid {
    pub fn create_mine_positions(&mut self, num_mines: u32) {
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
    
        positions.shuffle(&mut rng);
    
        self.mine_positions = positions.into_iter().take(num_mines as usize).collect::<HashSet<_>>()
    }

    pub fn get_num_mines_around(&self, x: u32, y: u32) -> u32 {
        let width = self.width;
        let height = self.height;
        let mine_positions = &self.mine_positions;

        let mut num_mines_around = 0;
        // 1 2 3
        // 4 x 5
        // 6 7 8

        // case 1
        if x > 1 && y > 1 && mine_positions.contains(&(x - 1, y - 1)) {
            num_mines_around += 1;
        }
        // case 2
        if y > 1 && mine_positions.contains(&(x, y - 1)) {
            num_mines_around += 1;
        }
        // case 3
        if x < width && y > 1 && mine_positions.contains(&(x + 1, y - 1)) {
            num_mines_around += 1;
        }
        // case 4
        if x > 1 && mine_positions.contains(&(x - 1, y)) {
            num_mines_around += 1;
        }
        // case 5
        if x < width && mine_positions.contains(&(x + 1, y)) {
            num_mines_around += 1;
        }
        // case 6
        if x > 1 && y < height && mine_positions.contains(&(x - 1, y + 1)) {
            num_mines_around += 1;
        }
        // case 7
        if y < height && mine_positions.contains(&(x, y + 1)) {
            num_mines_around += 1;
        }
        // case 8
        if x < width && y < height && mine_positions.contains(&(x + 1, y + 1)) {
            num_mines_around += 1;
        }

        num_mines_around
    }
}

#[derive(Resource)]
pub struct TotalMine(pub u32);

impl Default for TotalMine {
    fn default() -> Self {
        Self(99)
    }
}

#[derive(Resource, Default)]
pub struct RemainingMine(pub u32);