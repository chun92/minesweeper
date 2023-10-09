use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_egui::{egui::{self, PointerButton}, EguiContexts, EguiPlugin};

use crate::system::difficulty::Difficulty;
use crate::system::state::{GameState, MenuGameState, MenuInfoState, WindowState, RankingWindowState};
use crate::system::window::{init_window, init_window_with_ui};
use crate::system::auth::{Config, initiate_google_login};
use crate::system::uuid::UuidResource;
use crate::system::firestore::LoginDone;

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
            .add_state::<WindowState>()
            .add_state::<RankingWindowState>()
            .add_plugins(EguiPlugin)
            .add_systems(Startup, configure_visuals_system)
            .add_systems(Update, ui_system)
            .add_systems(OnEnter(WindowState::Opened), init_window_with_ui)
            .add_systems(OnEnter(WindowState::Closed), init_window)
            .add_systems(OnEnter(RankingWindowState::Opened), init_window_with_ui)
            .add_systems(OnEnter(RankingWindowState::Closed), init_window);
    }
}

fn configure_visuals_system(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub struct UiLocal {
    pub is_about_open: bool,
    pub is_ranking_open: bool,
}

impl Default for UiLocal {
    fn default() -> Self {
        Self {
            is_about_open: false,
            is_ranking_open: false,
        }
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut app_exit_events: ResMut<Events<AppExit>>,
    mut game_state: ResMut<NextState<GameState>>,
    current_game_menu_state: Res<State<MenuGameState>>,
    mut next_game_menu_state: ResMut<NextState<MenuGameState>>,
    current_info_menu_state: Res<State<MenuInfoState>>,
    mut next_info_menu_state: ResMut<NextState<MenuInfoState>>,
    current_window_state: Res<State<WindowState>>,
    mut next_window_state: ResMut<NextState<WindowState>>,
    mut difficulty: ResMut<Difficulty>,
    mut ui_local_values: Local<UiLocal>,
    mut ranking_difficulty: Local<Difficulty>,
    mut ui_size: ResMut<UiSize>,
    config: Res<Config>,
    uuid: Res<UuidResource>,
    login_done: Res<LoginDone>,
) {
    let ctx = contexts.ctx_mut();

    if ui_local_values.is_about_open {
        next_info_menu_state.set(MenuInfoState::Opened);
        if *current_window_state == WindowState::Closed {
            next_window_state.set(WindowState::Opened);
        }
    } else {
        if *current_window_state == WindowState::Opened {
            next_window_state.set(WindowState::Closed);
        }
    }

    egui::Window::new("About Minesweeper")
    .vscroll(true)
    .open(&mut ui_local_values.is_about_open)
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
                ui.hyperlink("https://github.com/chun92/minesweeper");
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

    if ui_local_values.is_ranking_open {
        next_info_menu_state.set(MenuInfoState::Opened);
        if *current_window_state == WindowState::Closed {
            next_window_state.set(WindowState::Opened);
        }
    } else {
        if *current_window_state == WindowState::Opened {
            next_window_state.set(WindowState::Closed);
        }
    }
    
    egui::Window::new("Ranking")
    .vscroll(true)
    .open(&mut ui_local_values.is_ranking_open)
    .show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(&mut *ranking_difficulty, Difficulty::Easy, "easy");
            ui.selectable_value(&mut *ranking_difficulty, Difficulty::Normal, "normal");
            ui.selectable_value(&mut *ranking_difficulty, Difficulty::Hard, "hard");
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
                let is_login_done = *login_done.done.lock().unwrap();

                if is_login_done {
                    let id = login_done.id.lock().unwrap();
                    if let Some(id) = id.as_ref() {
                        ui.label(format!("{}", id));
                    } else {
                        ui.label("Login: Error");
                    }
                } else {
                    if ui.button("Login").clicked() {
                        let config = &config;
                        let uuid = uuid.uuid.to_string();                    
                        initiate_google_login(config, uuid.as_str());
                        ui.close_menu();
                        next_game_menu_state.set(MenuGameState::Closed);
                    }
                }

                ui.separator();

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
                    ui_local_values.is_about_open = true;
                    ui.close_menu();
                    next_info_menu_state.set(MenuInfoState::Closed);
                }

                if ui.button("Ranking").clicked() {
                    ui_local_values.is_ranking_open = true;
                    ui.close_menu();
                    next_info_menu_state.set(MenuInfoState::Closed);
                }
            });

            info_menu.response.clicked_by(PointerButton::Primary).then(|| {
                if *current_info_menu_state == MenuInfoState::Opened && (!ui_local_values.is_about_open && !ui_local_values.is_ranking_open) {
                    next_info_menu_state.set(MenuInfoState::Closed);
                } else {
                    next_info_menu_state.set(MenuInfoState::Opened);
                }
            });

            info_menu.response.clicked_elsewhere().then(|| {
                if *current_info_menu_state == MenuInfoState::Opened && (!ui_local_values.is_about_open && !ui_local_values.is_ranking_open) {
                    next_info_menu_state.set(MenuInfoState::Closed);
                }
            });
        });
    });
}