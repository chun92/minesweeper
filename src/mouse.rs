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

pub fn print_mouse_events_system(
    buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    mut q_cells: Query<(&mut Cell, &Clickable)>,
) {
    if buttons.pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            for (mut cell, clickable) in q_cells.iter_mut() {
                if clickable.is_inside(position) {
                    if cell.is_right_pressed && !cell.is_both_pressed {
                        cell.both_pressed()
                    } else if !cell.is_left_pressed {
                        cell.left_pressed()
                    }
                } else {
                    if cell.is_both_pressed {
                        cell.both_out()
                    } else if cell.is_left_pressed {
                        cell.left_out()
                    }                   
                }
            }
        }
    }

    if buttons.pressed(MouseButton::Right) {
        if let Some(position) = q_windows.single().cursor_position() {
            for (mut cell, clickable) in q_cells.iter_mut() {
                if clickable.is_inside(position) {
                    if cell.is_left_pressed && !cell.is_both_pressed {
                        cell.both_pressed()
                    } else if !cell.is_right_pressed {
                        cell.right_pressed()
                    }
                } else {
                    if cell.is_both_pressed {
                        cell.both_out()
                    } else if cell.is_right_pressed {
                        cell.right_out()
                    }
                }
            }
        }
    }

    if buttons.just_released(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            for (mut cell, clickable) in q_cells.iter_mut() {
                if clickable.is_inside(position) {
                    if cell.is_both_pressed && !cell.is_right_pressed {
                        cell.both_released()
                    } else if cell.is_left_pressed {
                        cell.left_released()
                    }
                }
            }
        }
    }

    if buttons.just_released(MouseButton::Right) {
        if let Some(position) = q_windows.single().cursor_position() {
            for (mut cell, clickable) in q_cells.iter_mut() {
                if clickable.is_inside(position) {
                    if cell.is_both_pressed && !cell.is_left_pressed {
                        cell.both_released()
                    } else if cell.is_right_pressed {
                        cell.right_released()
                    }
                }
            }
        }
    }
}