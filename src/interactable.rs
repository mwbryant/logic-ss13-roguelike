use bevy::prelude::*;

use crate::{grid::Grid, menu::OpenMenu, player::PlayerInteract};

#[derive(Component, Default)]
pub enum Interactable {
    #[default]
    VendingMachine,
}

pub fn player_interact(
    mut interact: EventReader<PlayerInteract>,
    grid: Res<Grid<Interactable>>,
    mut open_menu: EventWriter<OpenMenu>,
) {
    for event in interact.read() {
        grid[&event.0].as_ref().map(|entities| {
            entities.iter().for_each(|entity| {
                // TODO if multiple make player select
                info!("Player interacted with me");
                open_menu.send(OpenMenu(*entity));
            });
        });
    }
}
