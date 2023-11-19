use bevy::{ecs::system::Command, prelude::*};

use crate::{
    graphics::{BorderDirection, GameSprite, TintOverride},
    grid::{GRID_SIZE_X, GRID_SIZE_Y},
    SCREEN_SIZE_X, SCREEN_SIZE_Y, TILE_SIZE,
};

pub const LOG_SIZE_X: usize = SCREEN_SIZE_X - GRID_SIZE_X;
pub const LOG_SIZE_Y: usize = SCREEN_SIZE_Y;

#[derive(Resource, Default)]
pub struct Log {
    entries: Vec<String>,
    entities: Vec<Vec<Entity>>,
}

#[derive(Component)]
pub struct LogCharacter {
    pub entry_index: usize,
    pub character_index: usize,
}

pub struct AddToLog(pub String, pub Option<TintOverride>);

impl Command for AddToLog {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world: &mut World, mut log: Mut<Log>| {
            let entry_index = log.entries.len();
            let entities = self
                .0
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    let entity = world
                        .spawn((
                            LogCharacter {
                                entry_index,
                                character_index: i,
                            },
                            SpatialBundle::default(),
                            GameSprite::Text(c),
                        ))
                        .id();
                    if let Some(tint) = &self.1 {
                        world.entity_mut(entity).insert(tint.clone());
                    }
                    entity
                })
                .collect();
            log.entries.push(self.0.clone());
            log.entities.push(entities);
        })
    }
}

pub fn lock_to_log(
    mut commands: Commands,
    mut characters: Query<&mut Transform, With<LogCharacter>>,
    log: Res<Log>,
) {
    let mut row = LOG_SIZE_Y as isize - 2;
    for (r, (i, entry)) in log.entries.iter().enumerate().rev().enumerate() {
        row -= 1;
        for (j, _char) in entry.chars().enumerate() {
            let entity = log.entities[i][j];
            if row >= 0 {
                if let Ok(mut transform) = characters.get_mut(entity) {
                    transform.translation = Vec3::new(
                        (GRID_SIZE_X + j + 1) as f32 * TILE_SIZE,
                        (GRID_SIZE_Y as f32 - r as f32 - 2.) * TILE_SIZE,
                        500.0,
                    );
                }
            } else {
                // PERF this sux
                if characters.get(entity).is_ok() {
                    commands.entity(entity).despawn_recursive();
                }
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
