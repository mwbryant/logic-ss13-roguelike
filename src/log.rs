use bevy::prelude::*;

use crate::{
    graphics::{BorderDirection, GameSprite},
    grid::{GRID_SIZE_X, GRID_SIZE_Y},
    SCREEN_SIZE_X, SCREEN_SIZE_Y, TILE_SIZE,
};

pub const LOG_SIZE_X: usize = SCREEN_SIZE_X - GRID_SIZE_X;
pub const LOG_SIZE_Y: usize = SCREEN_SIZE_Y;

pub fn setup_log(mut commands: Commands) {
    use BorderDirection::*;

    for x in 1..LOG_SIZE_X - 1 {
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                (GRID_SIZE_X + x) as f32 * TILE_SIZE,
                (GRID_SIZE_Y - 1) as f32 * TILE_SIZE,
                500.0,
            )),
            GameSprite::Border(Top),
        ));
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                (GRID_SIZE_X + x) as f32 * TILE_SIZE,
                (GRID_SIZE_Y as f32 - LOG_SIZE_Y as f32) * TILE_SIZE,
                500.0,
            )),
            GameSprite::Border(Bottom),
        ));
    }
    for y in 1..LOG_SIZE_Y - 1 {
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                (GRID_SIZE_X) as f32 * TILE_SIZE,
                (GRID_SIZE_Y as f32 - 1. - y as f32) * TILE_SIZE,
                500.0,
            )),
            GameSprite::Border(Left),
        ));
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                (GRID_SIZE_X + LOG_SIZE_X - 1) as f32 * TILE_SIZE,
                (GRID_SIZE_Y as f32 - 1. - y as f32) * TILE_SIZE,
                500.0,
            )),
            GameSprite::Border(Right),
        ));
    }
}
