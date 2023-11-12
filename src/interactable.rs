use bevy::prelude::*;

use crate::{grid::Grid, player::PlayerInteract};

#[derive(Component, Default)]
pub struct Interactable;

pub fn player_interact(mut interact: EventReader<PlayerInteract>, grid: Res<Grid<Interactable>>) {
    for event in interact.read() {
        grid[&event.0].as_ref().map(|entities| {
            entities.iter().for_each(|entity| {
                // TODO if multiple make player select
                info!("Player interacted with me");
            });
        });
    }
}
