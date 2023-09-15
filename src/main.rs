use bevy::prelude::*;

pub mod asset {
    pub mod loader;
    pub mod texture_type;
}

pub mod grid;
pub mod cell;
pub mod spawn;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<asset::loader::TextureAtlasResource>()
        .init_resource::<grid::Grid>()
        .add_systems(Startup, asset::loader::setup)
        .add_systems(PostStartup, spawn::spawn_camera)
        .add_systems(PostStartup, spawn::spawn_cells)
        .run();
}
