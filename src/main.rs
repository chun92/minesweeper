pub mod asset {
    pub mod loader;
    pub mod texture_type;
}

pub mod component {
    pub mod cell;
    pub mod smile;
    pub mod number;
    pub mod mine;
    pub mod grid;
    pub mod frame;
}

pub mod system {
    pub mod game_state;
    pub mod game_difficulty;
    pub mod mouse;
    pub mod timer;
    pub mod egui;
}

pub mod core {
    pub mod init {
        pub mod camera;
        pub mod grid;
    }
    pub mod update {
        pub mod cells;
        pub mod mines;
        pub mod smiles;
        pub mod time;
    }
}

use bevy::prelude::*;
use system::game_state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<asset::loader::TextureAtlasResource>()
        .init_resource::<system::timer::platform::Timer>()
        .init_resource::<component::mine::TotalMine>()
        .init_resource::<component::mine::RemainingMine>()
        .init_resource::<component::grid::Grid>()
        .add_systems(Startup, asset::loader::setup)
        .add_systems(PostStartup, core::init::camera::init)
        .add_systems(OnEnter(GameState::Init), (
            core::init::grid::clear,
            core::init::grid::init.after(core::init::grid::clear),
        ))
        .add_systems(Update, (
            system::mouse::mouse_events_system.after(system::egui::ui_system),
            core::update::smiles::update,
            core::update::time::update,
        ))
        .add_systems(OnEnter(GameState::Ready), (
            core::update::cells::reset,
            core::update::time::reset,
        ))
        .add_systems(Update, (
            core::update::cells::texture_for_ready,
            core::update::cells::first_click.after(core::update::cells::texture_for_ready),
            core::update::mines::update_for_ready.after(core::update::cells::first_click),
        )
        .run_if(in_state(GameState::Ready)))
        .add_systems(OnEnter(GameState::Playing), core::update::time::start)
        .add_systems(Update, (
            core::update::cells::update,
            core::update::cells::texture_for_playing.after(core::update::cells::update),
            core::update::mines::update_for_playing.after(core::update::cells::texture_for_playing),
        ).run_if(in_state(GameState::Playing)))
        .add_systems(OnEnter(GameState::Defeated), (
            core::update::cells::bomb,
            core::update::cells::texture_for_defeat.after(core::update::cells::bomb),
            core::update::time::stop,
            core::update::smiles::set_defeat,
        ))
        .add_systems(OnEnter(GameState::Win), (
            core::update::cells::texture_for_win,
            core::update::time::stop,
            core::update::smiles::set_win,
        ))
        .insert_resource(system::game_difficulty::Difficulty::Hard)
        .add_plugins(system::egui::EguiMenuPlugin)
        .run();
}
