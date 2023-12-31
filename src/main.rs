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
    pub mod game;
    pub mod state;
    pub mod difficulty;
    pub mod mouse;
    pub mod timer;
    pub mod egui;
    pub mod window;
    pub mod uuid;
    pub mod auth;
    pub mod firestore;
}

pub mod utils {
    pub mod yaml;
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
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(system::game::GamePlugin)
        .add_plugins(system::egui::EguiMenuPlugin)
        .add_plugins(system::auth::AuthPlugin)
        .add_plugins(system::firestore::FirestorePlugin)
        .run();
}
