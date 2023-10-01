use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResolution};
use crate::component::grid::{Grid, MARGIN_LEFT, MARGIN_RIGHT, MARGIN_UP, MARGIN_DOWN};
use crate::system::egui::{TOP_BAR_HEIGHT, UiSize};

pub fn init_window(
    grid: Res<Grid>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>
) {
    let window_size = grid.grid_window_size;
    q_windows.single_mut().title = "Minesweeper".to_string();
    q_windows.single_mut().resizable = false;
    let base_scale_factor = q_windows.single_mut().resolution.base_scale_factor();
    let mut resolution = WindowResolution::new(window_size.x + MARGIN_LEFT + MARGIN_RIGHT, window_size.y + MARGIN_UP + MARGIN_DOWN + TOP_BAR_HEIGHT);
    resolution.set_scale_factor(base_scale_factor);
    q_windows.single_mut().resolution = resolution;
}

pub fn init_window_with_ui(
    grid: Res<Grid>,
    ui_size: Res<UiSize>,
    mut q_windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    let window_size = grid.grid_window_size;
    let width = window_size.x.max(ui_size.width);
    let height = window_size.y.max(ui_size.height);
    let base_scale_factor = q_windows.single_mut().resolution.base_scale_factor();
    let mut resolution = WindowResolution::new(width + MARGIN_LEFT + MARGIN_RIGHT, height + MARGIN_UP + MARGIN_DOWN + TOP_BAR_HEIGHT);
    resolution.set_scale_factor(base_scale_factor);
    q_windows.single_mut().resolution = resolution;
}
