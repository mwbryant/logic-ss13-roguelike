use bevy::prelude::*;

use crate::{
    graphics::Impassable,
    grid::{Grid, GridLocation, LockToGrid},
    hands::Hands,
    interactable::Interactable,
    log::AddToLog,
    status_bar::UpdateStatusBar,
    usuable::PlayerUsed,
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
    mut commands: Commands,
    mut player: Query<&mut Hands, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::X) {
        let Ok(mut hands) = player.get_single_mut() else {
            return;
        };
        hands.swap_active(&mut commands);
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
            commands.entity(entity).insert((LockToGrid, grid.clone()));
            hands.clear_active();
            commands.add(UpdateStatusBar);
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
                return;
            }
        } else {
            player.combining = hands.get_active_held();
            if player.combining.is_some() {
                commands.add(AddToLog("Starting Combination".to_string(), None));
            }
        }
    }
}
