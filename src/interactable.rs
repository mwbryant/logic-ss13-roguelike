use bevy::prelude::*;

use crate::{
    grid::Grid,
    menu::{CentralMenu, CloseMenu, OpenMenu},
    player::{Player, PlayerInteract},
    Hands, Tool,
};

#[derive(Component, Default)]
pub enum Interactable {
    #[default]
    VendingMachine,
}

#[derive(Component, Default)]
pub struct VendingMachine {
    // Wiring insdie if panel is open
    // every ship has a master wiring diagram somewhere
    selection: usize,
}

pub fn vending_machine_menu(
    mut commands: Commands,
    menu: Res<CentralMenu>,
    mut machines: Query<&mut VendingMachine>,
    input: Res<Input<KeyCode>>,
    mut player_hand: Query<&mut Hands, With<Player>>,
    mut close_menu: EventWriter<CloseMenu>,
) {
    // TODO remove unwrap
    if let Ok(mut machine) = machines.get_mut(menu.owner.unwrap()) {
        if input.just_pressed(KeyCode::Return) {
            info!("Selected {:?}", machine.selection);
            let mut hands = player_hand.single_mut();
            if hands.can_pickup() {
                info!("Got screwdriver");
                let entity = commands.spawn(Tool::Screwdriver).id();
                hands.pickup(entity);
            }

            close_menu.send(CloseMenu);
        }
        if input.just_pressed(KeyCode::W) {
            machine.selection += 1;
        }
    }
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
