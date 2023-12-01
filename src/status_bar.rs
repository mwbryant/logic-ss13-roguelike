use bevy::{ecs::system::Command, prelude::*};

use crate::{
    graphics::{BorderDirection, GameSprite, TintOverride},
    grid::{GRID_SIZE_X, GRID_SIZE_Y},
    hands::Hands,
    player::Player,
    text::{AsciiText, SpawnText},
    SCREEN_SIZE_Y, TILE_SIZE,
};

pub const STATUS_SIZE_X: usize = GRID_SIZE_X + 1;
pub const STATUS_SIZE_Y: usize = SCREEN_SIZE_Y - GRID_SIZE_Y;

pub struct UpdateStatusBar;

impl Command for UpdateStatusBar {
    fn apply(self, world: &mut World) {
        world.resource_scope(|world, mut bar: Mut<StatusBar>| {
            for entity in bar.contents.drain(..) {
                world.entity_mut(entity).despawn_recursive();
            }
            let Ok(player_hands) = world
                .query_filtered::<&Hands, With<Player>>()
                .get_single(world)
                .cloned()
            else {
                return;
            };
            for (i, hand) in player_hands.hands.iter().cloned().enumerate() {
                let mut text = "Empty".to_string();
                if let Some(Ok(name)) = hand
                    .holding
                    .map(|entity| world.query::<&Name>().get(world, entity).cloned())
                {
                    text = name.to_string();
                }
                let entity = world.spawn_empty().id();
                let tint = if i == player_hands.active.unwrap() {
                    Some(TintOverride(Color::YELLOW))
                } else {
                    None
                };
                SpawnText {
                    text: AsciiText {
                        text: text.to_string(),
                        line_length: 99,
                    },
                    tint,
                    entity: Some(entity),
                    position: Vec3::new(TILE_SIZE * 3.0, (-(i as f32) - 2.0) * TILE_SIZE, 500.0),
                }
                .apply(world);
                bar.contents.push(entity);
                if let Some(item) = hand.holding {
                    let mut transform = world
                        .query::<&mut Transform>()
                        .get_mut(world, item)
                        .unwrap();
                    transform.translation =
                        Vec3::new(TILE_SIZE * 1.0, (-(i as f32) - 2.0) * TILE_SIZE, 500.0);
                }
            }
        });
    }
}

#[derive(Resource, Default)]
pub struct StatusBar {
    contents: Vec<Entity>,
}

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
