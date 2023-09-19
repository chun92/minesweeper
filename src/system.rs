use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use super::grid::Grid;
use super::grid::TotalMine;
use super::cell::Cell;
use super::cell::Cells;
use super::asset;
use super::mouse;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_cell(
    commands: &mut Commands,
    cell: Cell,
    grid: &mut Grid,
    texture_atlas_resource: &asset::loader::TextureAtlasResource,
    grid_id: Entity,
) {
    let position = cell.get_position(grid);
    let texture_atlas_handle = texture_atlas_resource.handles.get(&asset::texture_type::TextureAtlasType::Cells).unwrap();
    let index = cell.get_texture_index();
    
    let width = asset::texture_type::TextureAtlasType::Cells.get_cell_size().0;
    let height = asset::texture_type::TextureAtlasType::Cells.get_cell_size().1;
    
    let x = cell.x;
    let y = cell.y;
    let id = commands.spawn((
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
        mouse::Clickable(
            Vec3::new(position.x + grid.window_position.x, -position.y + grid.window_position.y, 0.0), 
            width, 
            height),
    )).set_parent(grid_id).id();

    grid.cells.push((x, y, id));
}

pub fn init_grid(
    mut commands: Commands,
    mines: Res<TotalMine>,
    mut grid: ResMut<Grid>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    texture_atlas_resource: Res<asset::loader::TextureAtlasResource>,
) {
    let window_width = q_windows.single().physical_width() as f32;
    let window_height = q_windows.single().physical_height() as f32;
    grid.init(30, 16, window_width, window_height);
    grid.create_mine_positions(mines.0);
    let mine_positions = &grid.mine_positions.clone();
    
    let cells_id = commands.spawn((
        Cells::new(),
        SpatialBundle::default()
    )).id();

    for x in 1..=grid.width {
        for y in 1..=grid.height {
            let is_mine = mine_positions.contains(&(x, y));
            let num_mines_around = grid.get_num_mines_around(x, y);
            spawn_cell(&mut commands, Cell::new(x, y, is_mine, num_mines_around), &mut grid, &texture_atlas_resource, cells_id);
        }
    }
}

fn update_cells_open(
    q_cells: &mut Query<(Entity, &mut Cell)>,
    grid: &Res<Grid>,
) {
    let mut queue: Vec<(Entity, u32, u32)> = Vec::new();
    let mut visitied: Vec<(u32, u32)> = Vec::new();
    let mut target: Vec<Entity> = Vec::new();
    for (entity, mut cell,) in q_cells.iter_mut() {
        if cell.is_opening {
            cell.is_opening = false;
            queue.push((entity, cell.x, cell.y));
            visitied.push((cell.x, cell.y));
        }
    }

    while let Some((entity, x, y)) = queue.pop() {
        target.push(entity);
        let query = q_cells.get(entity).unwrap();
        if query.1.is_mine {
            continue;
        }

        if query.1.num_mines_around != 0 {
            continue;
        }

        let arround_cells = grid.get_arround_cells(x, y);

        for (x, y, entity) in arround_cells {
            if !visitied.contains(&(x, y)) {
                if let Some(entity) = entity {
                    queue.push((entity, x, y));
                    visitied.push((x, y));
                }
            }
        }
    }
    
    for (entity, mut cell,) in q_cells.iter_mut() {
        if target.contains(&entity) {
            cell.open();
        }
    }
}

fn update_querying_cell(
    x: u32,
    y: u32,
    entity: Entity,
    q_cells: &mut Query<(Entity, &mut Cell)>,
    grid: &Res<Grid>,
) {
    let query = q_cells.get(entity).unwrap();
    let cell = query.1;
    if cell.is_mine {
        return;
    }
    if cell.num_mines_around == 0 {
        return;
    }
    let arround_cells = grid.get_arround_cells(x, y);
    for (_, _, entity) in arround_cells {
        if let Some(entity) = entity {
            let mut cell = q_cells.get_mut(entity).unwrap().1;
            if cell.state == super::cell::CellState::Hidden {
                cell.state = super::cell::CellState::Pressed;
            }
        }
    }
}

fn update_querying_out_cell(
    x: u32,
    y: u32,
    entity: Entity,
    q_cells: &mut Query<(Entity, &mut Cell)>,
    grid: &Res<Grid>,
) {
    let query = q_cells.get(entity).unwrap();
    let cell = query.1;
    if cell.is_mine {
        return;
    }
    if cell.num_mines_around == 0 {
        return;
    }
    let arround_cells = grid.get_arround_cells(x, y);

    for (_, _, entity) in arround_cells {
        if let Some(entity) = entity {
            let mut cell = q_cells.get_mut(entity).unwrap().1;
            if cell.state == super::cell::CellState::Pressed && !cell.is_left_pressed {
                cell.state = super::cell::CellState::Hidden;
            }
        }
    }
}

fn update_querying_done_cell(
    x: u32,
    y: u32,
    entity: Entity,
    q_cells: &mut Query<(Entity, &mut Cell)>,
    grid: &Res<Grid>,
) {
    let query = q_cells.get(entity).unwrap();
    let cell = query.1;
    if cell.is_mine {
        return;
    }
    if cell.num_mines_around == 0 {
        return;
    }
    let arround_cells = grid.get_arround_cells(x, y);
    let mut num_of_flagged = 0;

    for (_, _, entity) in &arround_cells {
        if let Some(entity) = entity {
            let cell = q_cells.get(*entity).unwrap().1;
            if cell.state == super::cell::CellState::Flagged {
                num_of_flagged += 1;
            }
        }
    }

    let open_others = num_of_flagged == cell.num_mines_around;

    for (_, _, entity) in &arround_cells {
        if let Some(entity) = entity {
            let mut cell = q_cells.get_mut(*entity).unwrap().1;
            if cell.state == super::cell::CellState::Pressed {
                cell.state = super::cell::CellState::Hidden;
                if open_others {
                    cell.is_opening = true;
                }
            } else if cell.state == super::cell::CellState::Flagged {
                if open_others {
                    cell.is_opening = true;
                }
            }
        }
    }
}

fn update_cells_query(
    q_cells: &mut Query<(Entity, &mut Cell)>,
    grid: &Res<Grid>,
) {
    let mut querying_queue: Vec<(Entity, u32, u32)> = Vec::new();
    let mut querying_out_queue: Vec<(Entity, u32, u32)> = Vec::new();
    let mut querying_done_queue: Vec<(Entity, u32, u32)> = Vec::new();
    for (entity, mut cell,) in q_cells.iter_mut() {
        if cell.query_state == super::cell::QueryState::Querying {
            querying_queue.push((entity, cell.x, cell.y));
        } else if cell.query_state == super::cell::QueryState::QueryingOut {
            querying_out_queue.push((entity, cell.x, cell.y));
        } else if cell.query_state == super::cell::QueryState::QueryingDone {
            querying_done_queue.push((entity, cell.x, cell.y));
        }
        cell.query_state = super::cell::QueryState::None;
    }

    if !querying_out_queue.is_empty() {
        let (entity, x, y) = querying_out_queue.pop().unwrap();
        update_querying_out_cell(x, y, entity, q_cells, grid);
    }
    if !querying_queue.is_empty() {
        let (entity, x, y) = querying_queue.pop().unwrap();
        update_querying_cell(x, y, entity, q_cells, grid);
    }
    if !querying_done_queue.is_empty() {
        let (entity, x, y) = querying_done_queue.pop().unwrap();
        update_querying_done_cell(x, y, entity, q_cells, grid);
    }

}

pub fn update_cells(
    mut q_cells: Query<(Entity, &mut Cell)>,
    grid: Res<Grid>,
) {
    update_cells_query(&mut q_cells, &grid);
    update_cells_open(&mut q_cells, &grid);
}


pub fn update_cells_texture(
    mut q_cells: Query<(&Cell, &mut TextureAtlasSprite)>,
) {
    for (cell, mut sprite) in q_cells.iter_mut() {
        sprite.index = cell.get_texture_index() as usize;
    }
}