use bevy::prelude::*;

use super::grid::Grid;
use super::grid::TotalMine;
use super::grid::RemainingMine;
use super::cell::Cell;
use super::asset;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_cell(
    commands: &mut Commands,
    cell: Cell,
    grid: &Grid,
    texture_atlas_resource: &asset::loader::TextureAtlasResource,
) {
    let position = cell.get_position(grid);
    let texture_atlas_handle = texture_atlas_resource.handles.get(&asset::texture_type::TextureAtlasType::Cells).unwrap();
    let index = if cell.is_mine {
        asset::texture_type::CellType::Mine as u32
    } else {
        asset::texture_type::CellType::get_revealed_num(cell.num_mines_around) as u32
    };
    commands.spawn((
        cell,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(index as usize),
            transform: Transform {
                translation: position,
                ..default()
            },
            ..default()
        }
    ));
}

pub fn spawn_cells(
    mut commands: Commands,
    mines: Res<TotalMine>,
    mut grid: ResMut<Grid>,
    texture_atlas_resource: Res<asset::loader::TextureAtlasResource>,
) {
    grid.create_mine_positions(mines.0);
    let mine_positions = &grid.mine_positions;
    for x in 1..=grid.width {
        for y in 1..=grid.height {
            let is_mine = mine_positions.contains(&(x, y));
            let num_mines_around = grid.get_num_mines_around(x, y);
            spawn_cell(&mut commands, Cell::new(x, y, is_mine, num_mines_around), &grid, &texture_atlas_resource);
        }
    }
}