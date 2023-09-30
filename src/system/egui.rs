use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_egui::{egui::{self, PointerButton}, EguiContexts, EguiPlugin};

use crate::system::game_difficulty::Difficulty;
use crate::system::game_state::{GameState, MenuGameState, MenuInfoState};

pub struct EguiMenuPlugin;

pub const TOP_BAR_HEIGHT: f32 = 20.0;

impl Plugin for EguiMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            .add_state::<MenuGameState>()
            .add_state::<MenuInfoState>()
            .add_plugins(EguiPlugin)
            .add_systems(Startup, configure_visuals_system)
            .add_systems(Update, ui_system);
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
    mut difficulty: ResMut<Difficulty>,
    mut is_about_open: Local<bool>,
) {
    let ctx = contexts.ctx_mut();

    if *is_about_open {
        next_info_menu_state.set(MenuInfoState::Opened);
    }

    egui::Window::new("About")
        .vscroll(true)
        .open(&mut is_about_open)
        .show(ctx, |ui| {
            ui.label("Windows can be moved by dragging them.");
            ui.label("They are automatically sized based on contents.");
            ui.label("You can turn on resizing and scrolling if you like.");
            ui.label("You would normally chose either panels OR windows.");
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