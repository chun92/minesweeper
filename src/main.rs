use bevy::prelude::*;
use game_state::GameState;

pub mod asset {
    pub mod loader;
    pub mod texture_type;
}

pub mod grid;
pub mod cell;
pub mod system;
pub mod mouse;
pub mod number;
pub mod game_state;
pub mod window;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .init_resource::<asset::loader::TextureAtlasResource>()
        .init_resource::<number::TotalMine>()
        .init_resource::<number::RemainingMine>()
        .init_resource::<grid::Grid>()
        .add_systems(Startup, asset::loader::setup)
        .add_systems(PostStartup, system::spawn_camera)
        .add_systems(PostStartup, system::init_grid)
        .add_systems(Update, mouse::mouse_events_system)
        .add_systems(Update, (
            system::update_cells,
            system::update_cells_texture.after(system::update_cells),
            system::update_mines.after(system::update_cells_texture),
        ).run_if(in_state(GameState::Ready).or_else(
            in_state(GameState::Playing)
        ))
        )
        .add_systems(OnEnter(GameState::Defeated), (
            system::bomb,
            system::update_cells_texture.after(system::bomb)
        ))
        .run();
}
