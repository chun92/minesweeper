use bevy::prelude::*;
use crate::component::cell::{Cell, CellState};
use crate::component::number::{self, NumberType, NumberTypeComponent, NumberIndex, NumberIndexComponent};
use crate::component::mine::{TotalMine, RemainingMine};

fn update_mines(
    q_cells: Query<&Cell>,
    mut q_mines: Query<(&NumberTypeComponent, &NumberIndexComponent, &mut TextureAtlasSprite)>,
    total_mine: Res<TotalMine>,
    mut remaining_mine: ResMut<RemainingMine>,
) {
    let mut num_of_flagged = 0;
    for cell in q_cells.iter() {
        if cell.state == CellState::Flagged || cell.state == CellState::WrongFlagged {
            num_of_flagged += 1;
        }
    }
    let cur_left_mine = total_mine.0 as i32 - num_of_flagged;
    remaining_mine.0 = cur_left_mine;
    let (first, second, third) = number::get_number_sprites(remaining_mine.0);
    for (number_type, number_index, mut sprite) in q_mines.iter_mut() {
        if number_type.0 != NumberType::MineCount {
            continue;
        }
        if number_type.0 == NumberType::MineCount && number_index.0 == NumberIndex::First {
            sprite.index = first as usize;
        } else if number_type.0 == NumberType::MineCount && number_index.0 == NumberIndex::Second {
            sprite.index = second as usize;
        } else if number_type.0 == NumberType::MineCount && number_index.0 == NumberIndex::Third {
            sprite.index = third as usize;
        }
    }        
}

pub fn update_for_playing(
    q_cells: Query<&Cell>,
    q_mines: Query<(&NumberTypeComponent, &NumberIndexComponent, &mut TextureAtlasSprite)>,
    total_mine: Res<TotalMine>,
    remaining_mine: ResMut<RemainingMine>,
) {
    update_mines(q_cells, q_mines, total_mine, remaining_mine);
}

pub fn update_for_ready(
    q_cells: Query<&Cell>,
    q_mines: Query<(&NumberTypeComponent, &NumberIndexComponent, &mut TextureAtlasSprite)>,
    total_mine: Res<TotalMine>,
    remaining_mine: ResMut<RemainingMine>,
) {
    update_mines(q_cells, q_mines, total_mine, remaining_mine);
}
