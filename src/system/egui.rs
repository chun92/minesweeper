use bevy::prelude::*;
use bevy::app::AppExit;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

use crate::system::{game_state::GameState, game_difficulty::Difficulty};

pub struct EguiMenuPlugin;

pub const TOP_BAR_HEIGHT: f32 = 20.0;

impl Plugin for EguiMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            // .insert_resource(Msaa::Sample4)
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
    mut difficulty: ResMut<Difficulty>,
) {
    let ctx = contexts.ctx_mut();

    egui::TopBottomPanel::top("top_panel")
        .exact_height(TOP_BAR_HEIGHT)
        .frame(egui::Frame::dark_canvas(&ctx.style()))
        .show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.set_height(TOP_BAR_HEIGHT);
            egui::menu::menu_button(ui, "File", |ui| {                
                if ui.selectable_label(*difficulty == Difficulty::Easy, "Easy").clicked() {
                    *difficulty = Difficulty::Easy;
                    game_state.set(GameState::Init);
                    ui.close_menu();
                }

                if ui.selectable_label(*difficulty == Difficulty::Normal, "Normal").clicked() {
                    *difficulty = Difficulty::Normal;
                    game_state.set(GameState::Init);
                    ui.close_menu();
                }

                if ui.selectable_label(*difficulty == Difficulty::Hard, "Hard").clicked() {
                    *difficulty = Difficulty::Hard;
                    game_state.set(GameState::Init);
                    ui.close_menu();
                }

                ui.separator();

                if ui.button("Quit").clicked() {
                    app_exit_events.send(AppExit);
                }
            });
        });
    });
}
