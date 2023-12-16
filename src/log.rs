use bevy::{ecs::system::Command, prelude::*};

use crate::{graphics::TintOverride, grid::GRID_SIZE_X, SCREEN_TILE_SIZE_X, SCREEN_TILE_SIZE_Y};

pub const LOG_SIZE_X: usize = SCREEN_TILE_SIZE_X - GRID_SIZE_X;
pub const LOG_SIZE_Y: usize = SCREEN_TILE_SIZE_Y;

#[derive(Resource, Default)]
pub struct Log {
    pub entries: Vec<String>,
}

pub struct AddToLog(pub String, pub Option<TintOverride>);

impl Command for AddToLog {
    fn apply(self, world: &mut World) {
        world.resource_scope(|_world: &mut World, mut log: Mut<Log>| {
            log.entries.push(self.0.clone());
        })
    }
}
