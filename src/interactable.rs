use bevy::prelude::*;

use crate::{
    graphics::TintOverride,
    grid::{Grid, GridLocation},
    hands::GiveItem,
    log::AddToLog,
    menu::{CentralMenu, CloseMenu, MenuRedraw, OpenMenu},
    player::{Player, PlayerInteract},
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
    pub selection: usize,
    pub options: Vec<Entity>,
}

pub fn update_vending_machine_menu_graphics(
    mut commands: Commands,
    mut menu: ResMut<CentralMenu>,
    machines: Query<&VendingMachine>,
    mut event: EventReader<MenuRedraw>,
    names: Query<&Name>,
) {
    for _ev in event.read() {
        if let Ok(machine) = machines.get(menu.owner.unwrap()) {
            menu.clear_menu(&mut commands);
            for (i, entry) in machine.options.iter().enumerate() {
                let name = names.get(*entry).unwrap();
                if i == machine.selection {
                    menu.set_row_text(&mut commands, name, i, Some(TintOverride(Color::YELLOW)));
                } else {
                    menu.set_row_text(&mut commands, name, i, None);
                }
            }
        }
    }
}

pub fn vending_machine_menu(
    mut commands: Commands,
    menu: Res<CentralMenu>,
    player: Query<&GridLocation, With<Player>>,
    mut machines: Query<&mut VendingMachine>,
    input: Res<Input<KeyCode>>,
    mut close_menu: EventWriter<CloseMenu>,
    mut redraw_menu: EventWriter<MenuRedraw>,
    mut give_item: EventWriter<GiveItem>,
    names: Query<&Name>,
) {
    // TODO remove unwrap
    if let Ok(mut machine) = machines.get_mut(menu.owner.unwrap()) {
        if input.just_pressed(KeyCode::Return) {
            let selection = machine.selection;
            if selection >= machine.options.len() {
                close_menu.send(CloseMenu);
                return;
            }
            let player_location = player.single();
            let entity = machine.options.remove(selection);
            let name = names.get(entity).unwrap();
            commands.add(AddToLog(format!("Dispensed {}", name).to_string(), None));
            machine.selection = 0;

            commands
                .entity(entity)
                .insert((player_location.clone(), Visibility::Visible));
            give_item.send(GiveItem {
                receiver: None,
                item: entity,
            });

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
