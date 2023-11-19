use bevy::prelude::*;

use crate::{
    graphics::{BorderDirection, GameSprite},
    grid::{GRID_SIZE_X, GRID_SIZE_Y},
    SCREEN_SIZE_Y, TILE_SIZE,
};

pub const STATUS_SIZE_X: usize = GRID_SIZE_X + 1;
pub const STATUS_SIZE_Y: usize = SCREEN_SIZE_Y - GRID_SIZE_Y;

pub fn setup_status_bar(mut commands: Commands) {
    use BorderDirection::*;
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(0.0, -1.0 * TILE_SIZE, 500.0)),
        GameSprite::Border(TopLeft),
    ));
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            0.0,
            -(STATUS_SIZE_Y as f32) * TILE_SIZE,
            500.0,
        )),
        GameSprite::Border(BottomLeft),
    ));

    for x in 1..STATUS_SIZE_X - 1 {
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                x as f32 * TILE_SIZE,
                -1.0 * TILE_SIZE,
                500.0,
            )),
            GameSprite::Border(Top),
        ));
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                x as f32 * TILE_SIZE,
                -(STATUS_SIZE_Y as f32) * TILE_SIZE,
                500.0,
            )),
            GameSprite::Border(Bottom),
        ));
    }
    for y in 1..STATUS_SIZE_Y - 1 {
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                0.0,
                -(y as f32 + 1.0) * TILE_SIZE,
                500.0,
            )),
            GameSprite::Border(Left),
        ));
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_xyz(
                (GRID_SIZE_X + STATUS_SIZE_X - 1) as f32 * TILE_SIZE,
                (GRID_SIZE_Y as f32 - 1. - y as f32) * TILE_SIZE,
                500.0,
            )),
            GameSprite::Border(Right),
        ));
    }
}
