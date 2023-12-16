use bevy::prelude::*;

use crate::{
    graphics::{Impassable, TintOverride},
    grid::{Grid, GridLocation, LockToGrid},
    hands::{GiveItem, Hands},
    interactable::Interactable,
    log::AddToLog,
    menu::{CentralMenu, CloseMenu, MenuRedraw, OpenMenu},
    usuable::PlayerUsed,
    Item,
};
#[derive(Event)]
pub struct PlayerCombined(pub Entity, pub Entity);

#[derive(Event)]
pub struct PlayerTookTurn;

#[derive(Event)]
pub struct PlayerInteract(pub GridLocation);

#[derive(Component, Default)]
pub struct Player {
    pub combining: Option<Entity>,
}

pub fn move_player(
    mut player: Query<(&mut GridLocation, &Player)>,
    input: Res<Input<KeyCode>>,
    wall_grid: Res<Grid<Impassable>>,
    interact_grid: Res<Grid<Interactable>>,
    mut turn_event: EventWriter<PlayerTookTurn>,
    mut interact_event: EventWriter<PlayerInteract>,
) {
    for (mut location, player) in &mut player {
        // TODO run if condition and allow player to combine with things on grid
        if player.combining.is_some() {
            return;
        }

        let mut point = location.get_location();

        if input.just_pressed(KeyCode::W) {
            point.y += 1;
        } else if input.just_pressed(KeyCode::S) {
            point.y -= 1;
        } else if input.just_pressed(KeyCode::D) {
            point.x += 1;
        } else if input.just_pressed(KeyCode::A) {
            point.x -= 1;
        }

        if point != location.get_location()
            && !wall_grid.occupied(&point.into())
            && location.try_set_location(point).is_ok()
        {
            turn_event.send(PlayerTookTurn);
            return;
        }
        if point != location.get_location() && interact_grid.occupied(&point.into()) {
            turn_event.send(PlayerTookTurn);
            interact_event.send(PlayerInteract(point.into()));
            return;
        }
    }
}

pub fn update_active_hand(
    mut player: Query<&mut Hands, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::X) {
        let Ok(mut hands) = player.get_single_mut() else {
            return;
        };
        hands.swap_active();
    }
}

pub fn use_active_hand(
    player: Query<&Hands, With<Player>>,
    mut event: EventWriter<PlayerUsed>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Z) {
        let Ok(hands) = player.get_single() else {
            return;
        };
        if let Some(entity) = hands.get_active_held() {
            event.send(PlayerUsed(entity));
        }
    }
}

pub fn drop_active_hand(
    mut commands: Commands,
    mut player: Query<(&mut Hands, &mut Player, &GridLocation)>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Q) {
        let Ok((mut hands, mut player, grid)) = player.get_single_mut() else {
            error!("No player!");
            return;
        };
        if let Some(entity) = hands.get_active_held() {
            if player.combining == Some(entity) {
                player.combining = None;
            }
            commands.add(AddToLog("Dropping held item".to_string(), None));
            commands.entity(entity).insert((LockToGrid, grid.clone())).insert(Visibility::Inherited);
            hands.clear_active();
        }
    }
}

pub fn pickup_from_ground(
    mut commands: Commands,
    player: Query<(&Hands, &GridLocation), With<Player>>,
    mut give_event: EventWriter<GiveItem>,
    mut menu_event: EventWriter<OpenMenu>,
    grid: Res<Grid<Item>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::G) {
        let Ok((hands, location)) = player.get_single() else {
            error!("No player!");
            return;
        };
        if !hands.can_pickup() {
            info!("Can't pickup");
            commands.add(AddToLog("Can't pickup".to_string(), None));
            return;
        }
        if let Some(entities) = &grid[location] {
            info!("{:?}", entities);
            if entities.len() == 1 {
                commands.add(AddToLog("Picked up item".to_string(), None));
                give_event.send(GiveItem {
                    receiver: None,
                    item: entities[0],
                });
            } else {
                let menu = commands
                    .spawn(PickupMenu {
                        items: entities.clone(),
                        selection: 0,
                    })
                    .id();
                menu_event.send(OpenMenu(menu));
            }
        }
    }
}

#[derive(Component)]
pub struct PickupMenu {
    items: Vec<Entity>,
    selection: usize,
}

#[allow(clippy::too_many_arguments)]
pub fn pickup_menu(
    mut commands: Commands,
    menu: Res<CentralMenu>,
    mut pickup: Query<&mut PickupMenu>,
    input: Res<Input<KeyCode>>,
    mut close_menu: EventWriter<CloseMenu>,
    mut redraw_menu: EventWriter<MenuRedraw>,
    mut give_item: EventWriter<GiveItem>,
    names: Query<&Name>,
) {
    if let Ok(mut pickup) = pickup.get_mut(menu.owner.unwrap()) {
        if input.just_pressed(KeyCode::Return) {
            let selection = pickup.selection;
            if selection >= pickup.items.len() {
                close_menu.send(CloseMenu);
                commands.entity(menu.owner.unwrap()).despawn_recursive();
                return;
            }
            let entity = pickup.items[selection];
            let name = names.get(entity).unwrap();
            commands.add(AddToLog(format!("Picked up {}", name).to_string(), None));
            give_item.send(GiveItem {
                receiver: None,
                item: entity,
            });
            close_menu.send(CloseMenu);
            commands.entity(menu.owner.unwrap()).despawn_recursive();
        }
        if input.just_pressed(KeyCode::S) {
            pickup.selection += 1;
            redraw_menu.send(MenuRedraw);
        }
        if input.just_pressed(KeyCode::W) {
            pickup.selection = pickup.selection.saturating_sub(1);
            redraw_menu.send(MenuRedraw);
        }
    }
}

pub fn update_pickup_menu_graphics(
    mut commands: Commands,
    mut menu: ResMut<CentralMenu>,
    pickup: Query<&PickupMenu>,
    mut event: EventReader<MenuRedraw>,
    names: Query<&Name>,
) {
    for _ev in event.read() {
        if let Ok(pickup) = pickup.get(menu.owner.unwrap()) {
            menu.clear_menu(&mut commands);
            for (i, entry) in pickup.items.iter().enumerate() {
                let name = names.get(*entry).unwrap();
                if i == pickup.selection {
                    menu.set_row_text(&mut commands, name, i, Some(TintOverride(Color::YELLOW)));
                } else {
                    menu.set_row_text(&mut commands, name, i, None);
                }
            }
        }
    }
}

// Make click based
pub fn start_combination(
    mut commands: Commands,
    mut player: Query<(&Hands, &mut Player)>,
    mut event: EventWriter<PlayerCombined>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::C) {
        let Ok((hands, mut player)) = player.get_single_mut() else {
            error!("No player!");
            return;
        };
        // TODO clean this up
        if let Some(first) = player.combining {
            if let Some(second) = hands.get_active_held() {
                if first == second {
                    player.combining = None;
                    commands.add(AddToLog("Cancel Combination".to_string(), None));
                    return;
                }
                commands.add(AddToLog("Combined with hand".to_string(), None));
                event.send(PlayerCombined(first, second));
                player.combining = None;
            } else {
                player.combining = None;
                commands.add(AddToLog("Cancel Combination".to_string(), None));
            }
        } else {
            player.combining = hands.get_active_held();
            if player.combining.is_some() {
                commands.add(AddToLog("Starting Combination".to_string(), None));
            }
        }
    }
}
