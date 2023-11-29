use bevy::{prelude::*, render::camera::ScalingMode, utils::HashMap};

use crate::{grid::GRID_SIZE_Y, SCREEN_SIZE_X, SCREEN_SIZE_Y, TILE_SIZE};

#[derive(Component, Default, Clone, Copy)]
pub struct Impassable;

#[derive(Component, Default, Hash, PartialEq, Eq, Debug)]
pub enum GameSprite {
    #[default]
    Player,
    Npc,
    Wall,
    MenuBackground,
    Floor,
    VendingMachine,
    Text(char),
    Border(BorderDirection),
}

#[derive(Component, Hash, PartialEq, Eq, Debug)]
pub enum BorderDirection {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Component, Clone)]
pub struct TintOverride(pub Color);

#[derive(Resource, Default)]
pub struct SpriteMap {
    map: HashMap<GameSprite, (Handle<TextureAtlas>, usize, Color)>,
}

pub fn update_sprites(
    mut commands: Commands,
    mut sprites: Query<
        (
            Entity,
            &GameSprite,
            Option<&mut TextureAtlasSprite>,
            Option<&TintOverride>,
        ),
        Or<(Changed<GameSprite>, Changed<TintOverride>)>,
    >,
    map: Res<SpriteMap>,
) {
    for (entity, sprite, texture_atlas, tint) in &mut sprites {
        let color = if let Some(tint) = tint {
            tint.0
        } else {
            map.map[sprite].2
        };

        match texture_atlas {
            Some(mut atlas) => {
                atlas.index = map.map[sprite].1;
                atlas.color = color;
            }
            None => {
                commands.entity(entity).insert((
                    TextureAtlasSprite {
                        index: map.map[sprite].1,
                        color,
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
            Color::rgba(1.0, 0.8, 0.5, 1.0),
        ),
    );
    map.map.insert(
        GameSprite::MenuBackground,
        (
            texture_atlas_handle.clone(),
            0,
            Color::rgba(0.2, 0.2, 0.2, 1.0),
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
    map.map.insert(
        GameSprite::Border(BorderDirection::Top),
        (
            texture_atlas_handle.clone(),
            12 * 16 + 13,
            Color::rgba(0.9, 0.9, 0.9, 1.0),
        ),
    );
    map.map.insert(
        GameSprite::Border(BorderDirection::Bottom),
        (
            texture_atlas_handle.clone(),
            12 * 16 + 13,
            Color::rgba(0.9, 0.9, 0.9, 1.0),
        ),
    );
    map.map.insert(
        GameSprite::Border(BorderDirection::Left),
        (
            texture_atlas_handle.clone(),
            11 * 16 + 10,
            Color::rgba(0.9, 0.9, 0.9, 1.0),
        ),
    );
    map.map.insert(
        GameSprite::Border(BorderDirection::Right),
        (
            texture_atlas_handle.clone(),
            11 * 16 + 10,
            Color::rgba(0.9, 0.9, 0.9, 1.0),
        ),
    );
    map.map.insert(
        GameSprite::Border(BorderDirection::BottomLeft),
        (
            texture_atlas_handle.clone(),
            12 * 16 + 8,
            Color::rgba(0.9, 0.9, 0.9, 1.0),
        ),
    );
    map.map.insert(
        GameSprite::Border(BorderDirection::TopRight),
        (
            texture_atlas_handle.clone(),
            11 * 16 + 11,
            Color::rgba(0.9, 0.9, 0.9, 1.0),
        ),
    );
    map.map.insert(
        GameSprite::Border(BorderDirection::BottomRight),
        (
            texture_atlas_handle.clone(),
            11 * 16 + 12,
            Color::rgba(0.9, 0.9, 0.9, 1.0),
        ),
    );
    map.map.insert(
        GameSprite::Border(BorderDirection::TopLeft),
        (
            texture_atlas_handle.clone(),
            12 * 16 + 9,
            Color::rgba(0.9, 0.9, 0.9, 1.0),
        ),
    );

    let alphabet = ('a'..='z').chain('A'..='Z').chain(" ><_-=+:;\"!".chars());

    for c in alphabet {
        map.map.insert(
            GameSprite::Text(c),
            (
                texture_atlas_handle.clone(),
                c as usize,
                Color::rgba(0.9, 0.9, 0.9, 1.0),
            ),
        );
    }

    commands.insert_resource(map);

    // TODO hdpi?
    let mut camera = Camera2dBundle::default();
    camera.projection.scaling_mode = ScalingMode::Fixed {
        width: 765.0,
        height: 432.0,
    };
    let center_x = (SCREEN_SIZE_X - 1) as f32 * TILE_SIZE / 2.0;
    let center_y = (SCREEN_SIZE_Y - 1) as f32 * TILE_SIZE / 2.0
        - (SCREEN_SIZE_Y - GRID_SIZE_Y) as f32 * TILE_SIZE;
    camera.transform = Transform::from_xyz(center_x, center_y, 0.0);
    commands.spawn(camera);
}
