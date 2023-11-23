use bevy::{ecs::system::Command, prelude::*};

use crate::{
    graphics::{GameSprite, TintOverride},
    TILE_SIZE,
};

#[derive(Component)]
pub struct AsciiText {
    pub text: String,
    pub line_length: usize,
}

#[derive(Component)]
pub struct Character {
    pub character_index: usize,
}

pub struct SpawnText {
    pub text: AsciiText,
    pub tint: Option<TintOverride>,
    pub entity: Option<Entity>,
    pub position: Vec3,
}

impl Command for SpawnText {
    fn apply(self, world: &mut World) {
        let entities = self
            .text
            .text
            .chars()
            .enumerate()
            .take(self.text.line_length)
            .map(|(i, c)| {
                let entity = world
                    .spawn((
                        Character { character_index: i },
                        SpatialBundle::from_transform(Transform::from_xyz(
                            i as f32 * TILE_SIZE,
                            0.0,
                            0.0,
                        )),
                        GameSprite::Text(c),
                    ))
                    .id();
                if let Some(tint) = &self.tint {
                    world.entity_mut(entity).insert(tint.clone());
                }
                entity
            })
            .collect::<Vec<_>>();
        // ugh
        if let Some(entity) = self.entity {
            world
                .entity_mut(entity)
                .insert((
                    self.text,
                    SpatialBundle::from_transform(Transform::from_translation(self.position)),
                ))
                .push_children(&entities);
        } else {
            world
                .spawn((
                    self.text,
                    SpatialBundle::from_transform(Transform::from_translation(self.position)),
                ))
                .push_children(&entities);
        }
    }
}
