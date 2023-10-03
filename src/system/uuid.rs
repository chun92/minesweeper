use bevy::prelude::*;
use uuid::Uuid;

#[derive(Resource)]
pub struct UuidResource {
    pub uuid: Uuid,
}

impl Default for UuidResource {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}