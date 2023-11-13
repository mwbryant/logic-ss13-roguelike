use bevy::prelude::*;

use crate::{grid::Grid, player::PlayerInteract, GameState};

#[derive(Component, Default)]
pub enum Interactable {
    #[default]
    VendingMachine,
}

pub fn player_interact(
    mut interact: EventReader<PlayerInteract>,
    grid: Res<Grid<Interactable>>,
    mut world: ResMut<GameState>,
) {
    for event in interact.read() {
        grid[&event.0].as_ref().map(|entities| {
            entities.iter().for_each(|entity| {
                // TODO if multiple make player select
                info!("Player interacted with me");
                world.in_menu = true;
            });
        });
    }
}
