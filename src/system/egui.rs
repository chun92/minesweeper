use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_egui::{egui::{self, PointerButton}, EguiContexts, EguiPlugin};

use crate::system::game_difficulty::Difficulty;
use crate::system::game_state::{GameState, MenuGameState, MenuInfoState, AboutWindowState};
use crate::system::window::{init_window, init_window_with_ui};

pub struct EguiMenuPlugin;

#[derive(Debug, Clone, Copy, Resource, Default)]
pub struct UiSize {
    pub width: f32,
    pub height: f32,
}

pub const TOP_BAR_HEIGHT: f32 = 20.0;

impl Plugin for EguiMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<UiSize>()
            .add_state::<MenuGameState>()
            .add_state::<MenuInfoState>()
            .add_state::<AboutWindowState>()
            .add_plugins(EguiPlugin)
            .add_systems(Startup, configure_visuals_system)
            .add_systems(Update, ui_system)
            .add_systems(OnEnter(AboutWindowState::Opened), init_window_with_ui)
            .add_systems(OnEnter(AboutWindowState::Closed), init_window);
    }
}

fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}


pub fn ui_system(
    mut contexts: EguiContexts,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut game_state: ResMut<NextState<GameState>>,
    current_game_menu_state: Res<State<MenuGameState>>,
    mut next_game_menu_state: ResMut<NextState<MenuGameState>>,
    current_info_menu_state: Res<State<MenuInfoState>>,
    mut next_info_menu_state: ResMut<NextState<MenuInfoState>>,
    current_about_window_state: Res<State<AboutWindowState>>,
    mut next_about_window_state: ResMut<NextState<AboutWindowState>>,
    mut difficulty: ResMut<Difficulty>,
    mut is_about_open: Local<bool>,
    mut ui_size: ResMut<UiSize>,
) {
    let ctx = contexts.ctx_mut();

    if *is_about_open {
        next_info_menu_state.set(MenuInfoState::Opened);
        if *current_about_window_state == AboutWindowState::Closed {
            next_about_window_state.set(AboutWindowState::Opened);
        }
    } else {
        if *current_about_window_state == AboutWindowState::Opened {
            next_about_window_state.set(AboutWindowState::Closed);
        }
    }

    egui::Window::new("About Minesweeper")
    .vscroll(true)
    .open(&mut is_about_open)
    .show(ctx, |ui| {
        ui.label("Minesweeper: Clone of a MS Minesweeper");
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.label("Version:");
                ui.monospace("0.1.0");
            });
            ui.horizontal(|ui| {
                ui.label("Developer:");
                ui.monospace("chun92");
            });
            ui.horizontal(|ui| {
                ui.label("License:");
                ui.monospace("MIT License");
            });
            ui.horizontal(|ui| {
                ui.label("Source Code:");
                if ui.hyperlink("https://github.com/chun92/minesweeper").clicked() {
                }
            });
            ui.horizontal(|ui| {
                ui.label("Game Engine:");
                ui.monospace("Bevy 0.11.2 with Rust");
            });
        });
        ui.group(|ui| {
            ui.label("Thank you for playing!");
        });

        if ui_size.width != ui.min_size().x {
            ui_size.width = ui.min_size().x;
        }

        if ui_size.height != ui.min_size().y {
            ui_size.height = ui.min_size().y;
        }
    });

    egui::TopBottomPanel::top("top_panel")
        .exact_height(TOP_BAR_HEIGHT)
        .frame(egui::Frame::dark_canvas(&ctx.style()))
        .show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.set_height(TOP_BAR_HEIGHT);
            let menu_game = egui::menu::menu_button(ui, "Game", |ui| {                
                if ui.selectable_label(*difficulty == Difficulty::Easy, "Easy").clicked() {
                    *difficulty = Difficulty::Easy;
                    game_state.set(GameState::Init);
                    ui.close_menu();
                    next_game_menu_state.set(MenuGameState::Closed);
                }

                if ui.selectable_label(*difficulty == Difficulty::Normal, "Normal").clicked() {
                    *difficulty = Difficulty::Normal;
                    game_state.set(GameState::Init);
                    ui.close_menu();
                    next_game_menu_state.set(MenuGameState::Closed);
                }

                if ui.selectable_label(*difficulty == Difficulty::Hard, "Hard").clicked() {
                    *difficulty = Difficulty::Hard;
                    game_state.set(GameState::Init);
                    ui.close_menu();
                    next_game_menu_state.set(MenuGameState::Closed);
                }

                ui.separator();

                if ui.button("Quit").clicked() {
                    app_exit_events.send(AppExit);
                }
            });

            menu_game.response.clicked_by(PointerButton::Primary).then(|| {
                if *current_game_menu_state == MenuGameState::Opened {
                    next_game_menu_state.set(MenuGameState::Closed);
                } else {
                    next_game_menu_state.set(MenuGameState::Opened);
                }
            });

            menu_game.response.clicked_elsewhere().then(|| {
                if *current_game_menu_state == MenuGameState::Opened {
                    next_game_menu_state.set(MenuGameState::Closed);
                }
            });

            let info_menu = egui::menu::menu_button(ui, "Info", |ui| {
                if ui.button("About").clicked() {
                    *is_about_open = true;
                    ui.close_menu();
                    next_info_menu_state.set(MenuInfoState::Closed);
                }
            });

            info_menu.response.clicked_by(PointerButton::Primary).then(|| {
                if *current_info_menu_state == MenuInfoState::Opened && !*is_about_open {
                    next_info_menu_state.set(MenuInfoState::Closed);
                } else {
                    next_info_menu_state.set(MenuInfoState::Opened);
                }
            });

            info_menu.response.clicked_elsewhere().then(|| {
                if *current_info_menu_state == MenuInfoState::Opened && !*is_about_open {
                    next_info_menu_state.set(MenuInfoState::Closed);
                }
            });
        });
    });
}