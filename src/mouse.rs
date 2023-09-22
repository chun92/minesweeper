//! Prints all mouse events to the console.

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use super::cell::Cell;

#[derive(Component)]
pub struct Clickable(pub Vec3, pub f32, pub f32);

impl Clickable {
    pub fn is_inside(&self, position: Vec2) -> bool {
        if position.x >= self.0.x - self.1 / 2.0 && position.x < self.0.x + self.1 / 2.0 &&
            position.y >= self.0.y - self.2 / 2.0 && position.y < self.0.y + self.2 / 2.0 {
            true
        } else {
            false
        }
    }
}

pub fn mouse_events_system(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_cells: Query<(&mut Cell, &Clickable)>,
    mut q_smiles: Query<(&mut super::smile::SmileComponent, &Clickable)>,
    mut next_state: ResMut<NextState<super::game_state::GameState>>,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            for (mut cell, clickable) in q_cells.iter_mut() {
                if clickable.is_inside(position) {
                    if !cell.is_left_pressed {
                        cell.left_pressed();
                    }
                } else {
                    if cell.is_left_pressed {
                        cell.left_out();
                    }                   
                }
            }

            for (mut smile, clickable) in q_smiles.iter_mut() {
                if clickable.is_inside(position) {
                    if !smile.is_pressed {
                        smile.pressed();
                    }
                }
            }
        }
    }

    if buttons.just_released(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            for (mut cell, clickable) in q_cells.iter_mut() {
                if clickable.is_inside(position) {
                    if cell.is_left_pressed {
                        cell.left_released();
                    }
                }
            }

            for (mut smile, _) in q_smiles.iter_mut() {
                if smile.is_pressed {
                    smile.released(&mut next_state);
                }
            }
        }
    }
    
    if buttons.just_pressed(MouseButton::Right) {
        if let Some(position) = q_windows.single().cursor_position() {
            for (mut cell, clickable) in q_cells.iter_mut() {
                if clickable.is_inside(position) {
                    cell.right_just_pressed();
                }
            }
        }
    }
}