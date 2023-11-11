use bevy::{prelude::*, utils::HashMap};

#[derive(Component, Default, Clone, Copy)]
pub struct Impassable;

#[derive(Component, Default, Hash, PartialEq, Eq)]
pub enum GameSprite {
    #[default]
    Player,
    Npc,
    Wall,
}

#[derive(Resource, Default)]
pub struct SpriteMap {
    map: HashMap<GameSprite, (Handle<TextureAtlas>, usize)>,
}

pub fn update_sprites(
    mut commands: Commands,
    mut sprites: Query<(Entity, &GameSprite, Option<&mut TextureAtlasSprite>)>,
    map: Res<SpriteMap>,
) {
    for (entity, sprite, texture_atlas) in &mut sprites {
        match texture_atlas {
            Some(mut atlas) => atlas.index = map.map[sprite].1,
            None => {
                commands.entity(entity).insert((
                    TextureAtlasSprite {
                        index: map.map[sprite].1,
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
    map.map
        .insert(GameSprite::Player, (texture_atlas_handle.clone(), 1));
    map.map
        .insert(GameSprite::Npc, (texture_atlas_handle.clone(), 2));
    map.map
        .insert(GameSprite::Wall, (texture_atlas_handle.clone(), 3 + 3 * 16));

    commands.insert_resource(map);

    // TODO hdpi?
    let mut camera = Camera2dBundle::default();
    camera.projection.scale = 0.3;
    commands.spawn(camera);
}
