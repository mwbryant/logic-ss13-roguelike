use bevy::prelude::*;

use crate::grid::GridLocation;

#[derive(Component)]
pub struct Player;
pub fn move_player(mut player: Query<&mut GridLocation, With<Player>>, input: Res<Input<KeyCode>>) {
    for mut location in &mut player {
        if input.just_pressed(KeyCode::W) {
            location.0.y += 1;
        }
        if input.just_pressed(KeyCode::S) {
            location.0.y -= 1;
        }
        if input.just_pressed(KeyCode::D) {
            location.0.x += 1;
        }
        if input.just_pressed(KeyCode::A) {
            location.0.x -= 1;
        }
    }
}
