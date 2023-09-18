use bevy::prelude::*;

pub mod asset {
    pub mod loader;
    pub mod texture_type;
}

pub mod grid;
pub mod cell;
pub mod system;
pub mod mouse;
pub mod window;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<asset::loader::TextureAtlasResource>()
        .init_resource::<grid::TotalMine>()
        .init_resource::<grid::RemainingMine>()
        .init_resource::<grid::Grid>()
        .add_systems(Startup, asset::loader::setup)
        .add_systems(PostStartup, system::spawn_camera)
        .add_systems(PostStartup, system::init_grid)
        .add_systems(Update, mouse::print_mouse_events_system)
        .add_systems(Update, system::open_cells)
        .run();
}
