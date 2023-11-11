use bevy::prelude::*;

use crate::{
    grid::{Grid, GridLocation},
    map::Impassable,
};

#[derive(Event)]
pub struct PlayerTookTurn;

#[derive(Component)]
pub struct Player;
pub fn move_player(
    mut player: Query<&mut GridLocation, With<Player>>,
    input: Res<Input<KeyCode>>,
    grid: Res<Grid<Impassable>>,
    mut turn_event: EventWriter<PlayerTookTurn>,
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
            && !grid.occupied(&point.into())
            && location.try_set_location(point).is_ok()
        {
            turn_event.send(PlayerTookTurn);
        }
    }
}
