use bevy::prelude::*;
use crate::component::grid::Grid;
use crate::component::cell::{Cell, CellState, QueryState};
use crate::component::mine::TotalMine;
use crate::system::state::GameState;

pub fn reset(
    mut q_cells: Query<&mut Cell>,
) {
    for mut cell in q_cells.iter_mut() {
        cell.reset();
    }
}

fn plant_mines_cells(
    q_cells: &mut Query<&mut Cell>,
    grid: &mut Grid,
) {
    let mine_positions = &grid.mine_positions.clone();
    for mut cell in q_cells.iter_mut() {
        let x = cell.x;
        let y = cell.y;
        let is_mine = mine_positions.contains(&(x, y));
        let num_mines_around = grid.get_num_mines_around(x, y);
        cell.change_mine(is_mine, num_mines_around)
    }
}

pub fn first_click(
    mut q_cells: Query<&mut Cell>,
    mut grid: ResMut<Grid>,
    mut next_state: ResMut<NextState<GameState>>,
    mines: Res<TotalMine>,
) {
    let mut queue: Vec<(u32, u32)> = Vec::new();
    for cell in q_cells.iter_mut() {
        if cell.is_opening {
            queue.push((cell.x, cell.y));
        }
    }

    if queue.len() == 0 {
        return;
    }

    let (x, y) = queue.pop().unwrap();
    grid.create_mine_positions(mines.0, Some((x, y)));
    plant_mines_cells(&mut q_cells, &mut grid);
    next_state.set(GameState::Playing);
}

fn update_cells_open(
    q_cells: &mut Query<(Entity, &mut Cell)>,
    grid: &Res<Grid>,
) -> bool {
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
    
    let mut result = true;
    for (entity, mut cell,) in q_cells.iter_mut() {
        if target.contains(&entity) {
            result &= cell.open();
        }
    }

    result
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
            if cell.state == CellState::Hidden {
                cell.state = CellState::Pressed;
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
            if cell.state == CellState::Pressed && !cell.is_left_pressed {
                cell.state = CellState::Hidden;
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
            if cell.state == CellState::Flagged {
                num_of_flagged += 1;
            }
        }
    }

    let open_others = num_of_flagged == cell.num_mines_around;

    for (_, _, entity) in &arround_cells {
        if let Some(entity) = entity {
            let mut cell = q_cells.get_mut(*entity).unwrap().1;
            if cell.state == CellState::Pressed {
                cell.state = CellState::Hidden;
                if open_others {
                    cell.is_opening = true;
                }
            } else if cell.state == CellState::Flagged {
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
        if cell.query_state == QueryState::Querying {
            querying_queue.push((entity, cell.x, cell.y));
        } else if cell.query_state == QueryState::QueryingOut {
            querying_out_queue.push((entity, cell.x, cell.y));
        } else if cell.query_state == QueryState::QueryingDone {
            querying_done_queue.push((entity, cell.x, cell.y));
        }
        cell.query_state = QueryState::None;
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

fn check_win(
    q_cells: &Query<(Entity, &mut Cell)>,
) -> bool {
    for (_, cell) in q_cells.iter() {
        if !(cell.state == CellState::Flagged || cell.state == CellState::Revealed) {
            return false;
        }
    }
    true
}

pub fn update(
    mut q_cells: Query<(Entity, &mut Cell)>,
    grid: Res<Grid>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    update_cells_query(&mut q_cells, &grid);
    let result = update_cells_open(&mut q_cells, &grid);
    if !result {
        next_state.set(GameState::Defeated);
    }
    if check_win(&q_cells) {
        next_state.set(GameState::Win);
    }
}

pub fn bomb(
    mut q_cells: Query<&mut Cell>,
) {
    for mut cell in q_cells.iter_mut() {
        cell.bomb();
    }
}

fn update_cells_texture(
    q_cells: &mut Query<(&Cell, &mut TextureAtlasSprite)>,
) {
    for (cell, mut sprite) in q_cells.iter_mut() {
        sprite.index = cell.get_texture_index() as usize;
    }
}

pub fn texture_for_playing(
    mut q_cells: Query<(&Cell, &mut TextureAtlasSprite)>,
) {
    update_cells_texture(&mut q_cells);
}

pub fn texture_for_defeat(
    mut q_cells: Query<(&Cell, &mut TextureAtlasSprite)>,
) {
    update_cells_texture(&mut q_cells);
}

pub fn texture_for_win(
    mut q_cells: Query<(&Cell, &mut TextureAtlasSprite)>,
) {
    update_cells_texture(&mut q_cells);
}

pub fn texture_for_ready(
    mut q_cells: Query<(&Cell, &mut TextureAtlasSprite)>,
) {
    update_cells_texture(&mut q_cells);
}
