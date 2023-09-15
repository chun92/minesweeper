use bevy::prelude::*;

use super::grid::Grid;
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
    let texture_atlas_handle = texture_atlas_resource.handles.get(&asset::texture_type::TextureAtlasType::Bombs).unwrap();
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
    grid: Res<Grid>,
    texture_atlas_resource: Res<asset::loader::TextureAtlasResource>,
) {
    for x in 1..=grid.width {
        for y in 1..=grid.height {
            spawn_cell(&mut commands, Cell::new(x, y), &grid, &texture_atlas_resource);
        }
    }
}