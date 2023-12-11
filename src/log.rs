use bevy::{ecs::system::Command, prelude::*};

use crate::{
    graphics::{BorderDirection, GameSprite, TintOverride},
    grid::{GRID_SIZE_X, GRID_SIZE_Y},
    text::{AsciiText, SpawnText},
    SCREEN_TILE_SIZE_X, SCREEN_TILE_SIZE_Y, TILE_SIZE,
};

pub const LOG_SIZE_X: usize = SCREEN_TILE_SIZE_X - GRID_SIZE_X;
pub const LOG_SIZE_Y: usize = SCREEN_TILE_SIZE_Y;

#[derive(Resource, Default)]
pub struct Log {
    entries: Vec<String>,
    entities: Vec<Entity>,
}

pub struct AddToLog(pub String, pub Option<TintOverride>);

impl Command for AddToLog {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world: &mut World, mut log: Mut<Log>| {
            let entry_index = log.entries.len();
            log.entries.push(self.0.clone());
            let text = world.spawn_empty().id();
            SpawnText {
                text: AsciiText {
                    text: self.0,
                    line_length: LOG_SIZE_X - 2,
                },
                tint: self.1,
                entity: Some(text),
                position: Vec3::new(
                    (GRID_SIZE_X + 1) as f32 * TILE_SIZE,
                    (GRID_SIZE_Y as f32 - entry_index as f32 - 2.) * TILE_SIZE,
                    500.0,
                ),
            }
            .apply(world);
            log.entities.push(text);
        })
    }
}

pub fn lock_to_log(
    mut commands: Commands,
    mut characters: Query<&mut Transform, With<AsciiText>>,
    log: Res<Log>,
) {
    let mut row = LOG_SIZE_Y as isize - 2;
    for (r, entity) in log.entities.iter().rev().enumerate() {
        row -= 1;
        if row >= 0 {
            if let Ok(mut transform) = characters.get_mut(*entity) {
                transform.translation = Vec3::new(
                    (GRID_SIZE_X + 1) as f32 * TILE_SIZE,
                    (GRID_SIZE_Y as f32 - r as f32 - 2.) * TILE_SIZE,
                    500.0,
                );
            }
        } else {
            // PERF this sux
            if characters.get(*entity).is_ok() {
                commands.entity(*entity).despawn_recursive();
            }
        }
    }
}

pub fn setup_log(mut commands: Commands) {
    use BorderDirection::*;
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            (GRID_SIZE_X) as f32 * TILE_SIZE,
            (GRID_SIZE_Y - 1) as f32 * TILE_SIZE,
            500.0,
        )),
        GameSprite::Border(TopLeft),
    ));
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            (LOG_SIZE_X + GRID_SIZE_X - 1) as f32 * TILE_SIZE,
            (GRID_SIZE_Y - 1) as f32 * TILE_SIZE,
            500.0,
        )),
        GameSprite::Border(TopRight),
    ));
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            (GRID_SIZE_X + LOG_SIZE_X - 1) as f32 * TILE_SIZE,
            (GRID_SIZE_Y as f32 - LOG_SIZE_Y as f32) * TILE_SIZE,
            500.0,
        )),
        GameSprite::Border(BottomRight),
    ));

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_xyz(
            (LOG_SIZE_X + GRID_SIZE_X - 1) as f32 * TILE_SIZE,
            (GRID_SIZE_Y - 1) as f32 * TILE_SIZE,
            500.0,
        )),
        GameSprite::Border(TopRight),
    ));

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
