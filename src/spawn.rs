use bevy::prelude::*;
use bevy::utils::HashSet;
use rand::seq::SliceRandom;

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
    commands.spawn((
        cell,
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
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
    grid: Res<Grid>,
    texture_atlas_resource: Res<asset::loader::TextureAtlasResource>,
) {
    let mine_positions = create_mine_positions(grid.width, grid.height, mines.0);
    for x in 1..=grid.width {
        for y in 1..=grid.height {
            let is_mine = mine_positions.contains(&(x, y));
            spawn_cell(&mut commands, Cell::new(x, y, is_mine), &grid, &texture_atlas_resource);
        }
    }
}

fn create_mine_positions(grid_width: u32, grid_height: u32, num_mines: u32) -> HashSet<(u32, u32)> {
    let mut rng = rand::thread_rng();
    
    let num_mines = if num_mines > grid_width * grid_height {
        grid_width * grid_height
    } else {
        num_mines
    };

    // Create a vector with all possible positions
    let mut positions: Vec<(u32, u32)> = (1..=grid_width)
        .flat_map(|x| (1..=grid_height).map(move |y| (x, y)))
        .collect();

    positions.shuffle(&mut rng);

    positions.into_iter().take(num_mines as usize).collect::<HashSet<_>>()
}