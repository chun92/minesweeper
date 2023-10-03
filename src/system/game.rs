use bevy::prelude::*;

use crate::asset;
use crate::component;
use crate::core;
use crate::system;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_state::<system::state::GameState>()
            .init_resource::<asset::loader::TextureAtlasResource>()
            .init_resource::<system::timer::platform::Timer>()
            .init_resource::<system::uuid::UuidResource>()
            .init_resource::<component::mine::TotalMine>()
            .init_resource::<component::mine::RemainingMine>()
            .init_resource::<component::grid::Grid>()
            .add_systems(Startup, asset::loader::setup)
            .add_systems(PostStartup, core::init::camera::init)
            .add_systems(OnEnter(system::state::GameState::Init), (
                core::init::grid::clear,
                core::init::grid::init.after(core::init::grid::clear),
                system::window::init_window.after(core::init::grid::init),
            ))
            .add_systems(Update, (
                system::mouse::mouse_events_system.after(system::egui::ui_system)
                    .run_if(system::state::run_if_all_menu_closed),
                core::update::smiles::update,
                core::update::time::update,
            ))
            .add_systems(OnEnter(system::state::GameState::Ready), (
                core::update::cells::reset,
                core::update::time::reset,
            ))
            .add_systems(Update, (
                core::update::cells::texture_for_ready,
                core::update::cells::first_click.after(core::update::cells::texture_for_ready),
                core::update::mines::update_for_ready.after(core::update::cells::first_click),
            )
            .run_if(in_state(system::state::GameState::Ready)))
            .add_systems(OnEnter(system::state::GameState::Playing), core::update::time::start)
            .add_systems(Update, (
                core::update::cells::update,
                core::update::cells::texture_for_playing.after(core::update::cells::update),
                core::update::mines::update_for_playing.after(core::update::cells::texture_for_playing),
            ).run_if(in_state(system::state::GameState::Playing)))
            .add_systems(OnEnter(system::state::GameState::Defeated), (
                core::update::cells::bomb,
                core::update::cells::texture_for_defeat.after(core::update::cells::bomb),
                core::update::time::stop,
                core::update::smiles::set_defeat,
            ))
            .add_systems(OnEnter(system::state::GameState::Win), (
                core::update::cells::texture_for_win,
                core::update::time::stop,
                core::update::smiles::set_win,
            ))
            .insert_resource(system::difficulty::Difficulty::Hard);
    }
}