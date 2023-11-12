use bevy::{prelude::*, render::camera::ScalingMode, utils::HashMap};

use crate::grid::{GRID_SIZE_X, GRID_SIZE_Y, TILE_SIZE};

#[derive(Component, Default, Clone, Copy)]
pub struct Impassable;

#[derive(Component, Default, Hash, PartialEq, Eq)]
pub enum GameSprite {
    #[default]
    Player,
    Npc,
    Wall,
    Floor,
    VendingMachine,
}

#[derive(Resource, Default)]
pub struct SpriteMap {
    map: HashMap<GameSprite, (Handle<TextureAtlas>, usize, Color)>,
}

pub fn update_sprites(
    mut commands: Commands,
    mut sprites: Query<(Entity, &GameSprite, Option<&mut TextureAtlasSprite>), Changed<GameSprite>>,
    map: Res<SpriteMap>,
) {
    for (entity, sprite, texture_atlas) in &mut sprites {
        match texture_atlas {
            Some(mut atlas) => atlas.index = map.map[sprite].1,
            None => {
                commands.entity(entity).insert((
                    TextureAtlasSprite {
                        index: map.map[sprite].1,
                        color: map.map[sprite].2,
                        ..default()
                    },
                    map.map[sprite].0.clone(),
                ));
            }
        }
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Ascii.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(9.0, 9.0),
        16,
        16,
        Some(Vec2::splat(2.0)),
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut map = SpriteMap::default();
    map.map.insert(
        GameSprite::Player,
        (texture_atlas_handle.clone(), 1, Color::WHITE),
    );
    map.map.insert(
        GameSprite::Npc,
        (texture_atlas_handle.clone(), 2, Color::WHITE),
    );
    map.map.insert(
        GameSprite::Wall,
        (
            texture_atlas_handle.clone(),
            3 + 3 * 16,
            Color::rgba(0.5, 0.5, 0.5, 1.0),
        ),
    );
    map.map.insert(
        GameSprite::Floor,
        (
            texture_atlas_handle.clone(),
            14 + 2 * 16,
            Color::rgba(0.5, 0.5, 0.5, 0.5),
        ),
    );
    map.map.insert(
        GameSprite::VendingMachine,
        (
            texture_atlas_handle.clone(),
            'V' as usize,
            Color::rgba(0.5, 0.9, 0.5, 0.5),
        ),
    );

    commands.insert_resource(map);

    // TODO hdpi?
    let mut camera = Camera2dBundle::default();
    // camera.projection.scale = 0.4;
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: 765.0,
        height: 432.0,
    };
    let center_x = (GRID_SIZE_X - 1) as f32 * TILE_SIZE / 2.0;
    let center_y = (GRID_SIZE_Y - 1) as f32 * TILE_SIZE / 2.0;
    camera.transform = Transform::from_xyz(center_x, center_y, 0.0);
    commands.spawn(camera);
}
