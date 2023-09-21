use bevy::prelude::*;
use bevy::window::PrimaryWindow;


use super::grid::Grid;
use super::cell::Cell;
use super::number::TotalMine;
use super::number::NumberType;
use super::number::NumberTypeComponent;
use super::number::NumberIndex;
use super::number::NumberIndexComponent;
use super::number::NumberSprite;
use super::asset;
use super::mouse;
use super::asset::texture_type::TextureType;

#[derive(Component, Default)]
pub struct Frame();

impl Frame {
    pub fn new() -> Self {
        Self::default()
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_cell(
    commands: &mut Commands,
    cell: Cell,
    grid: &mut Grid,
    texture_atlas_resource: &asset::loader::TextureAtlasResource,
    frame_id: Entity,
) {
    let position = cell.get_position(grid);
    let texture_atlas_handle = texture_atlas_resource.handles.get(&TextureType::Cells).unwrap();
    let index = cell.get_texture_index();
    
    let width = TextureType::Cells.get_cell_size().0;
    let height = TextureType::Cells.get_cell_size().1;
    
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
    )).set_parent(frame_id).id();

    grid.cells.push((x, y, id));
}

const FRAME_SYSTEM_HEIGHT : f32 = 33.0;
const EPSILON : f32 = 0.000000000001;

fn spawn_frame(
    commands: &mut Commands,
    grid: &mut Grid,
    texture_atlas_resource: &asset::loader::TextureAtlasResource,
    frame_id: Entity,
) {
    let width = grid.window_size.x;
    let height = grid.window_size.y;
    let left_position = -width / 2.0;
    let right_position = width / 2.0;
    let top_position = height / 2.0;
    let bottom_position = -height / 2.0;

    let mut spawn = |texture_type: TextureType, position: Vec3, scale: Vec3| {
        let texture_atlas_handle = texture_atlas_resource.handles.get(&texture_type).unwrap();
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform {
                    translation: position,
                    scale: scale,
                    ..default()
                },
                ..default()
            },
        )).set_parent(frame_id);
    };

    let position = Vec3::new(left_position - TextureType::EdgeLeft.get_cell_size().0 / 2.0, 0.0, 0.0);
    let scale = Vec3::new(1.0, height / TextureType::EdgeLeft.get_cell_size().1, 1.0);
    spawn(TextureType::EdgeLeft, position, scale);

    let position = Vec3::new(right_position + TextureType::EdgeRight.get_cell_size().0 / 2.0, 0.0, 0.0);
    let scale = Vec3::new(1.0, height / TextureType::EdgeRight.get_cell_size().1, 1.0);
    spawn(TextureType::EdgeRight, position, scale);    

    let position = Vec3::new(0.0, top_position + TextureType::EdgeTop.get_cell_size().1 / 2.0 - 1.0, 0.0);
    let scale = Vec3::new(width / TextureType::EdgeTop.get_cell_size().0, 1.0, 1.0);
    spawn(TextureType::EdgeTop, position, scale);

    let position = Vec3::new(0.0, bottom_position - TextureType::EdgeBottom.get_cell_size().1 / 2.0, 0.0);
    let scale = Vec3::new(width / TextureType::EdgeBottom.get_cell_size().0, 1.0, 1.0);
    spawn(TextureType::EdgeBottom, position, scale);

    let position = Vec3::new(left_position - TextureType::CornerLeftBottom.get_cell_size().0 / 2.0, bottom_position - TextureType::CornerLeftBottom.get_cell_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerLeftBottom, position, scale);

    let position = Vec3::new(right_position + TextureType::CornerRightBottom.get_cell_size().0 / 2.0, bottom_position - TextureType::CornerRightBottom.get_cell_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerRightBottom, position, scale);

    let position = Vec3::new(left_position - TextureType::CornerLeftTop.get_cell_size().0 / 2.0, top_position + TextureType::CornerLeftTop.get_cell_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerLeftTop, position, scale);

    let position = Vec3::new(right_position + TextureType::CornerRightTop.get_cell_size().0 / 2.0, top_position + TextureType::CornerRightTop.get_cell_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerRightTop, position, scale);

    let top_position = top_position + TextureType::CornerLeftTop.get_cell_size().1;
    let left_position = left_position - 1.0;
    let right_position = right_position + 1.0;

    let position = Vec3::new(left_position - TextureType::EdgeLeftUpper.get_cell_size().0 / 2.0, top_position + FRAME_SYSTEM_HEIGHT / 2.0, 0.0);
    let scale = Vec3::new(1.0, FRAME_SYSTEM_HEIGHT / TextureType::EdgeLeftUpper.get_cell_size().1, 1.0);
    spawn(TextureType::EdgeLeftUpper, position, scale); 

    let position = Vec3::new(right_position + TextureType::EdgeRightUpper.get_cell_size().0 / 2.0, top_position + FRAME_SYSTEM_HEIGHT / 2.0, 0.0);
    let scale = Vec3::new(1.0, FRAME_SYSTEM_HEIGHT / TextureType::EdgeRightUpper.get_cell_size().1, 1.0);
    spawn(TextureType::EdgeRightUpper, position, scale);

    let position = Vec3::new(0.0, top_position + FRAME_SYSTEM_HEIGHT / 2.0, 0.0);
    let scale = Vec3::new(width / TextureType::Background.get_cell_size().0, FRAME_SYSTEM_HEIGHT / TextureType::Background.get_cell_size().1, 0.0);
    spawn(TextureType::Background, position, scale);
    
    let top_position = top_position + FRAME_SYSTEM_HEIGHT;

    let position = Vec3::new(0.0, top_position + TextureType::EdgeTopUpper.get_cell_size().1 / 2.0, 0.0);
    let scale = Vec3::new((width + 2.0) / TextureType::EdgeTopUpper.get_cell_size().0, 1.0, 1.0);
    spawn(TextureType::EdgeTopUpper, position, scale);

    let position = Vec3::new(left_position - TextureType::CornerLeftUpperTop.get_cell_size().0 / 2.0, top_position + TextureType::CornerLeftUpperTop.get_cell_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerLeftUpperTop, position, scale);

    let position = Vec3::new(right_position + TextureType::CornerRightUpperTop.get_cell_size().0 / 2.0, top_position + TextureType::CornerRightUpperTop.get_cell_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerRightUpperTop, position, scale);

    let top_position = top_position - FRAME_SYSTEM_HEIGHT;
    
    let mut spawn = |position: Vec3, number_type: NumberType| {
        let texture_atlas_handle = texture_atlas_resource.handles.get(&TextureType::Number).unwrap();
        commands.spawn((
            NumberTypeComponent::new(number_type),
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(NumberSprite::One as usize),
                transform: Transform {
                    translation: position,
                    ..default()
                },
                ..default()
            },
        )).with_children(|commands| {
            let texture_atlas_handle = texture_atlas_resource.handles.get(&TextureType::Numbers).unwrap();
            commands.spawn((
                NumberTypeComponent::new(number_type),
                NumberIndexComponent::new(NumberIndex::First),
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(NumberSprite::Zero as usize),
                    transform: Transform {
                        translation: Vec3::new(-TextureType::Numbers.get_cell_size().0, 1.0 / 2.0, position.z * 2.0),
                        ..default()
                    },
                    ..default()
                },
            ));
            commands.spawn((
                NumberTypeComponent::new(number_type),
                NumberIndexComponent::new(NumberIndex::Second),
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(NumberSprite::Zero as usize),
                    transform: Transform {
                        translation: Vec3::new(0.0, 1.0 / 2.0, position.z * 2.0),
                        ..default()
                    },
                    ..default()
                },
            ));
            commands.spawn((
                NumberTypeComponent::new(number_type),
                NumberIndexComponent::new(NumberIndex::Third),
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(NumberSprite::Zero as usize),
                    transform: Transform {
                        translation: Vec3::new(TextureType::Numbers.get_cell_size().0,  1.0 / 2.0, position.z * 2.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }).set_parent(frame_id);
    };
    
    let position = Vec3::new(left_position + 6.0 + TextureType::Number.get_cell_size().0 / 2.0, top_position + TextureType::Number.get_cell_size().1 / 2.0 + 4.0, EPSILON);
    spawn(position, NumberType::MineCount);
    
    let position = Vec3::new(right_position - 8.0 - TextureType::Number.get_cell_size().0 / 2.0, top_position + TextureType::Number.get_cell_size().1 / 2.0 + 4.0, EPSILON);
    spawn(position, NumberType::Time);

    // let position = Vec3::new(0.0, top_position + TextureType::Smile.get_cell_size().1 / 2.0 + 3.0, EPSILON);
    // let scale = Vec3::new(1.0, 1.0, 1.0);
    // spawn(TextureType::Smile, position, scale);
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
    
    let frame_id = commands.spawn((
        Frame::new(),
        SpatialBundle::default()
    )).id();

    for x in 1..=grid.width {
        for y in 1..=grid.height {
            let is_mine = mine_positions.contains(&(x, y));
            let num_mines_around = grid.get_num_mines_around(x, y);
            spawn_cell(&mut commands, Cell::new(x, y, is_mine, num_mines_around), &mut grid, &texture_atlas_resource, frame_id);
        }
    }
    spawn_frame(&mut commands, &mut grid, &texture_atlas_resource, frame_id);
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
    mut next_state: ResMut<NextState<super::game_state::GameState>>,
) {
    update_cells_query(&mut q_cells, &grid);
    let result = update_cells_open(&mut q_cells, &grid);
    if !result {
        next_state.set(super::game_state::GameState::Defeated);
    }
}

pub fn update_cells_texture(
    mut q_cells: Query<(&Cell, &mut TextureAtlasSprite)>,
) {
    for (cell, mut sprite) in q_cells.iter_mut() {
        sprite.index = cell.get_texture_index() as usize;
    }
}

pub fn bomb(
    mut q_cells: Query<&mut Cell>,
) {
    for mut cell in q_cells.iter_mut() {
        cell.bomb();
    }
}

pub fn update_mines(
    q_cells: Query<&Cell>,
    mut q_mines: Query<(&NumberTypeComponent, &NumberIndexComponent, &mut TextureAtlasSprite)>,
    total_mine: Res<super::number::TotalMine>,
    mut remaining_mine: ResMut<super::number::RemainingMine>,
) {
    let mut num_of_flagged = 0;
    for cell in q_cells.iter() {
        if cell.state == super::cell::CellState::Flagged || cell.state == super::cell::CellState::WrongFlagged {
            num_of_flagged += 1;
        }
    }
    let cur_left_mine = total_mine.0 as i32 - num_of_flagged;
    if remaining_mine.0 != cur_left_mine {
        remaining_mine.0 = cur_left_mine;
        let (first, second, third) = super::number::get_number_sprites(remaining_mine.0);
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
}