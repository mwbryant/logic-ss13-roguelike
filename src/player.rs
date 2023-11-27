use bevy::prelude::*;

use crate::{
    graphics::Impassable,
    grid::{Grid, GridLocation},
    hands::Hands,
    interactable::Interactable,
    usuable::PlayerUsed,
};

#[derive(Event)]
pub struct PlayerTookTurn;

#[derive(Event)]
pub struct PlayerInteract(pub GridLocation);

#[derive(Component)]
pub struct Player;
pub fn move_player(
    mut player: Query<&mut GridLocation, With<Player>>,
    input: Res<Input<KeyCode>>,
    wall_grid: Res<Grid<Impassable>>,
    interact_grid: Res<Grid<Interactable>>,
    mut turn_event: EventWriter<PlayerTookTurn>,
    mut interact_event: EventWriter<PlayerInteract>,
) {
    for mut location in &mut player {
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
