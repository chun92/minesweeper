//! Prints all mouse events to the console.

use bevy::{input::mouse::MouseButtonInput, prelude::*};
use bevy::window::PrimaryWindow;
use super::cell::Cell;

#[derive(Component)]
pub struct Clickable(pub Vec3, pub f32, pub f32);

pub fn print_mouse_events_system(
    mut mouse_button_input_events: EventReader<MouseButtonInput>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    q_cells: Query<&Clickable>,
) {
    for event in mouse_button_input_events.iter() {
        if let Some(position) = q_windows.single().cursor_position() {
            info!("{:?}, Cursor is inside the primary window, at {:?}", event, position);
            // for clickable in q_cells.iter() {
            //     info!("{:?}", clickable.0);
            // }
        } else {
            info!("{:?}, Cursor is not in the game window.", event);
        }
    }
}