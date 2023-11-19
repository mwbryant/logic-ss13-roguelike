use bevy::prelude::*;

use crate::{
    graphics::TintOverride,
    grid::Grid,
    log::AddToLog,
    menu::{CentralMenu, CloseMenu, MenuRedraw, OpenMenu},
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
    // Wiring inside if panel is open
    // every ship has a master wiring diagram somewhere
    selection: usize,
}

pub fn update_vending_machine_menu_graphics(
    mut commands: Commands,
    mut menu: ResMut<CentralMenu>,
    machines: Query<&VendingMachine>,
    mut event: EventReader<MenuRedraw>,
) {
    for _ev in event.read() {
        let rows = [
            "> Screwdriver",
            "> Screwdriver",
            "> Screwdriver",
            "> Screwdriver",
        ];
        menu.clear_menu(&mut commands);
        if let Ok(machine) = machines.get(menu.owner.unwrap()) {
            for (i, row) in rows.iter().enumerate() {
                if i == machine.selection {
                    menu.set_row_text(&mut commands, row, i, Some(TintOverride(Color::YELLOW)));
                } else {
                    menu.set_row_text(&mut commands, row, i, None);
                }
            }
        }
    }
}

pub fn vending_machine_menu(
    mut commands: Commands,
    menu: Res<CentralMenu>,
    mut machines: Query<&mut VendingMachine>,
    input: Res<Input<KeyCode>>,
    mut player_hand: Query<&mut Hands, With<Player>>,
    mut close_menu: EventWriter<CloseMenu>,
    mut redraw_menu: EventWriter<MenuRedraw>,
) {
    // TODO remove unwrap
    if let Ok(mut machine) = machines.get_mut(menu.owner.unwrap()) {
        if input.just_pressed(KeyCode::Return) {
            info!("Selected {:?}", machine.selection);
            let mut hands = player_hand.single_mut();
            if hands.can_pickup() {
                info!("Got screwdriver");
                commands.add(AddToLog("Got screwdriver".to_string(), None));
                let entity = commands.spawn(Tool::Screwdriver).id();
                hands.pickup(entity);
            }

            close_menu.send(CloseMenu);
        }
        if input.just_pressed(KeyCode::S) {
            machine.selection += 1;
            redraw_menu.send(MenuRedraw);
        }
        if input.just_pressed(KeyCode::W) {
            machine.selection = machine.selection.saturating_sub(1);
            redraw_menu.send(MenuRedraw);
        }
    }
}

pub fn player_interact(
    mut interact: EventReader<PlayerInteract>,
    grid: Res<Grid<Interactable>>,
    mut open_menu: EventWriter<OpenMenu>,
) {
    for event in interact.read() {
        if let Some(entities) = grid[&event.0].as_ref() {
            entities.iter().for_each(|entity| {
                // TODO if multiple make player select
                info!("Player interacted with me");
                open_menu.send(OpenMenu(*entity));
            });
        }
    }
}
