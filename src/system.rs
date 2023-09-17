use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::grid::Grid;
use super::grid::TotalMine;
use super::grid::RemainingMine;
use super::cell::Cell;
use super::asset;
use super::mouse;

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
    let index = if cell.is_revealed {
            if cell.is_mine {
            asset::texture_type::CellType::Mine as u32
        } else {
            asset::texture_type::CellType::get_revealed_num(cell.num_mines_around) as u32
        }
    } else {
        asset::texture_type::CellType::Hidden as u32
    };
    
    let width = asset::texture_type::TextureAtlasType::Cells.get_cell_size().0;
    let height = asset::texture_type::TextureAtlasType::Cells.get_cell_size().1;
    
    commands.spawn((
        grid.clone(),
        SpatialBundle::default()
    )).with_children(|commands| {
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
            },
            mouse::Clickable(position, width, height),
        ));
    });
}

pub fn spawn_grid(
    mut commands: Commands,
    mines: Res<TotalMine>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    texture_atlas_resource: Res<asset::loader::TextureAtlasResource>,
) {
    let window_width = q_windows.single().physical_width() as f32;
    let window_height = q_windows.single().physical_height() as f32;
    let mut grid = Grid::new();
    grid.init(30, 16, window_width, window_height);
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