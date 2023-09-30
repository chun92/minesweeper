use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};

use crate::component::grid::{Grid, MARGIN_UP, MARGIN_DOWN, MARGIN_LEFT, MARGIN_RIGHT, MARGIN_X, MARGIN_Y};
use crate::component::cell::Cell;
use crate::component::smile::{SmileComponent, SmileSprite};
use crate::component::number::{NumberSprite, NumberType, NumberTypeComponent, NumberIndex, NumberIndexComponent};
use crate::component::mine::TotalMine;
use crate::component::frame::Frame;
use crate::system::egui::TOP_BAR_HEIGHT;
use crate::system::game_difficulty::Difficulty;
use crate::system::game_state::GameState;
use crate::system::mouse;
use crate::asset::{self, texture_type::TextureType};

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
    
    let width = TextureType::Cells.get_texture_size().0;
    let height = TextureType::Cells.get_texture_size().1;
    
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
            Vec3::new(position.x + grid.window_position.x, -position.y + grid.window_position.y + TOP_BAR_HEIGHT / 2.0, 0.0), 
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
    let width = grid.grid_window_size.x;
    let height = grid.grid_window_size.y;
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
                    translation: Vec3::new(position.x + MARGIN_X, position.y + 1.0 + MARGIN_Y - TOP_BAR_HEIGHT / 2.0, position.z),
                    scale: scale,
                    ..default()
                },
                ..default()
            },
        )).set_parent(frame_id);
    };

    let position = Vec3::new(left_position - TextureType::EdgeLeft.get_texture_size().0 / 2.0, 0.0, 0.0);
    let scale = Vec3::new(1.0, height / TextureType::EdgeLeft.get_texture_size().1, 1.0);
    spawn(TextureType::EdgeLeft, position, scale);

    let position = Vec3::new(right_position + TextureType::EdgeRight.get_texture_size().0 / 2.0, 0.0, 0.0);
    let scale = Vec3::new(1.0, height / TextureType::EdgeRight.get_texture_size().1, 1.0);
    spawn(TextureType::EdgeRight, position, scale);    

    let position = Vec3::new(0.0, top_position + TextureType::EdgeTop.get_texture_size().1 / 2.0 - 1.0, 0.0);
    let scale = Vec3::new(width / TextureType::EdgeTop.get_texture_size().0, 1.0, 1.0);
    spawn(TextureType::EdgeTop, position, scale);

    let position = Vec3::new(0.0, bottom_position - TextureType::EdgeBottom.get_texture_size().1 / 2.0, 0.0);
    let scale = Vec3::new(width / TextureType::EdgeBottom.get_texture_size().0, 1.0, 1.0);
    spawn(TextureType::EdgeBottom, position, scale);

    let position = Vec3::new(left_position - TextureType::CornerLeftBottom.get_texture_size().0 / 2.0, bottom_position - TextureType::CornerLeftBottom.get_texture_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerLeftBottom, position, scale);

    let position = Vec3::new(right_position + TextureType::CornerRightBottom.get_texture_size().0 / 2.0, bottom_position - TextureType::CornerRightBottom.get_texture_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerRightBottom, position, scale);

    let position = Vec3::new(left_position - TextureType::CornerLeftTop.get_texture_size().0 / 2.0, top_position + TextureType::CornerLeftTop.get_texture_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerLeftTop, position, scale);

    let position = Vec3::new(right_position + TextureType::CornerRightTop.get_texture_size().0 / 2.0, top_position + TextureType::CornerRightTop.get_texture_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerRightTop, position, scale);

    let top_position = top_position + TextureType::CornerLeftTop.get_texture_size().1;
    let left_position = left_position - 1.0;
    let right_position = right_position + 1.0;

    let position = Vec3::new(left_position - TextureType::EdgeLeftUpper.get_texture_size().0 / 2.0, top_position + FRAME_SYSTEM_HEIGHT / 2.0, 0.0);
    let scale = Vec3::new(1.0, FRAME_SYSTEM_HEIGHT / TextureType::EdgeLeftUpper.get_texture_size().1, 1.0);
    spawn(TextureType::EdgeLeftUpper, position, scale); 

    let position = Vec3::new(right_position + TextureType::EdgeRightUpper.get_texture_size().0 / 2.0, top_position + FRAME_SYSTEM_HEIGHT / 2.0, 0.0);
    let scale = Vec3::new(1.0, FRAME_SYSTEM_HEIGHT / TextureType::EdgeRightUpper.get_texture_size().1, 1.0);
    spawn(TextureType::EdgeRightUpper, position, scale);

    let position = Vec3::new(0.0, top_position + FRAME_SYSTEM_HEIGHT / 2.0, 0.0);
    let scale = Vec3::new(width / TextureType::Background.get_texture_size().0 + 2.0, FRAME_SYSTEM_HEIGHT / TextureType::Background.get_texture_size().1, -EPSILON);
    spawn(TextureType::Background, position, scale);
    
    let top_position = top_position + FRAME_SYSTEM_HEIGHT;

    let position = Vec3::new(0.0, top_position + TextureType::EdgeTopUpper.get_texture_size().1 / 2.0, 0.0);
    let scale = Vec3::new((width + 2.0) / TextureType::EdgeTopUpper.get_texture_size().0, 1.0, 1.0);
    spawn(TextureType::EdgeTopUpper, position, scale);

    let position = Vec3::new(left_position - TextureType::CornerLeftUpperTop.get_texture_size().0 / 2.0, top_position + TextureType::CornerLeftUpperTop.get_texture_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerLeftUpperTop, position, scale);

    let position = Vec3::new(right_position + TextureType::CornerRightUpperTop.get_texture_size().0 / 2.0, top_position + TextureType::CornerRightUpperTop.get_texture_size().1 / 2.0, 0.0);
    let scale = Vec3::new(1.0, 1.0, 1.0);
    spawn(TextureType::CornerRightUpperTop, position, scale);

    let top_position = top_position - FRAME_SYSTEM_HEIGHT;
    
    let mut spawn = |position: Vec3, number_type: NumberType| {
        let texture_atlas_handle = texture_atlas_resource.handles.get(&TextureType::Number).unwrap();
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform {
                    translation: Vec3::new(position.x + MARGIN_X, position.y + MARGIN_Y - TOP_BAR_HEIGHT / 2.0, position.z),
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
                        translation: Vec3::new(-TextureType::Numbers.get_texture_size().0, 0.0, position.z * 2.0),
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
                        translation: Vec3::new(0.0, 0.0, position.z * 2.0),
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
                        translation: Vec3::new(TextureType::Numbers.get_texture_size().0,  0.0, position.z * 2.0),
                        ..default()
                    },
                    ..default()
                },
            ));
        }).set_parent(frame_id);
    };
    
    let position = Vec3::new(left_position + 6.0 + TextureType::Number.get_texture_size().0 / 2.0, top_position + TextureType::Number.get_texture_size().1 / 2.0 + 4.0, EPSILON);
    spawn(position, NumberType::MineCount);
    
    let position = Vec3::new(right_position - 8.0 - TextureType::Number.get_texture_size().0 / 2.0, top_position + TextureType::Number.get_texture_size().1 / 2.0 + 4.0, EPSILON);
    spawn(position, NumberType::Time);
    
    let mut spawn = |position: Vec3| {
        let position = Vec3::new(position.x + MARGIN_X, position.y + MARGIN_Y - TOP_BAR_HEIGHT / 2.0, position.z);
        let texture_atlas_handle = texture_atlas_resource.handles.get(&TextureType::Smile).unwrap();
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(0),
                transform: Transform {
                    translation: position,
                    ..default()
                },
                ..default()
            },
        )).with_children(|commands| {
            let texture_atlas_handle = texture_atlas_resource.handles.get(&TextureType::Smiles).unwrap();
            commands.spawn((
                SmileComponent::new(),
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(SmileSprite::Normal as usize),
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, position.z * 2.0),
                        ..default()
                    },
                    ..default()
                },
                mouse::Clickable(
                    Vec3::new(position.x + grid.window_position.x, -position.y + grid.window_position.y + TOP_BAR_HEIGHT / 2.0, 0.0), 
                        TextureType::Smiles.get_texture_size().0, 
                        TextureType::Smiles.get_texture_size().1),
            ));
        }).set_parent(frame_id);
    };
    
    let position = Vec3::new(0.0, top_position + TextureType::Smile.get_texture_size().1 / 2.0 + 3.0, EPSILON);
    spawn(position);
}

fn get_difficulty(difficulty: &Difficulty) -> (u32, u32, u32) {
    match difficulty {
        Difficulty::Easy => (9, 9, 10),
        Difficulty::Normal => (16, 16, 40),
        Difficulty::Hard => (30, 16, 99),
    }
}

fn spawn_grid(
    mines: &mut TotalMine,
    difficulty: &Difficulty,
    commands: &mut Commands,
    grid: &mut Grid,
    q_windows: &mut Query<&mut Window, With<PrimaryWindow>>
) -> Entity {
    let (width, height, num_mines) = get_difficulty(difficulty);
    grid.init(width, height);
    mines.init(num_mines);
    let window_size = grid.grid_window_size;
    q_windows.single_mut().title = "Minesweeper".to_string();
    q_windows.single_mut().resizable = false;
    q_windows.single_mut().resolution = WindowResolution::new(window_size.x + MARGIN_LEFT + MARGIN_RIGHT, window_size.y + MARGIN_UP + MARGIN_DOWN + TOP_BAR_HEIGHT);
    grid.create_mine_positions(mines.0, None);
    
    commands.spawn((
        Frame::new(),
        SpatialBundle::default()
    )).id()
}

fn spawn_cells(
    commands: &mut Commands,
    grid: &mut Grid,
    texture_atlas_resource: &asset::loader::TextureAtlasResource,
    frame_id: Entity,
) {
    let mine_positions = &grid.mine_positions.clone();
    for x in 1..=grid.width {
        for y in 1..=grid.height {
            let is_mine = mine_positions.contains(&(x, y));
            let num_mines_around = grid.get_num_mines_around(x, y);
            spawn_cell(commands, Cell::new(x, y, is_mine, num_mines_around), grid, &texture_atlas_resource, frame_id);
        }
    }
}

pub fn init(
    mut commands: Commands,
    mut mines: ResMut<TotalMine>,
    mut grid: ResMut<Grid>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
    difficulty : Res<Difficulty>,
    texture_atlas_resource: Res<asset::loader::TextureAtlasResource>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let frame_id = spawn_grid(&mut mines, &difficulty, &mut commands, &mut grid, &mut q_windows);
    spawn_cells(&mut commands, &mut grid, &texture_atlas_resource, frame_id);
    spawn_frame(&mut commands, &mut grid, &texture_atlas_resource, frame_id);
    next_state.set(GameState::Ready);
}

pub fn clear(
    mut commands: Commands,
    to_despawn: Query<Entity, With<Frame>>,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}