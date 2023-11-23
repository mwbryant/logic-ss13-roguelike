use bevy::prelude::*;

use crate::{
    graphics::{GameSprite, TintOverride},
    grid::{GRID_SIZE_X, GRID_SIZE_Y},
    text::{AsciiText, SpawnText},
    TILE_SIZE,
};

pub const MENU_SIZE_X: usize = 48;
pub const MENU_SIZE_Y: usize = 24;

#[derive(Resource, Default)]
pub struct CentralMenu {
    pub open: bool,
    pub contents: Vec<Entity>,
    pub owner: Option<Entity>,
}

impl CentralMenu {
    pub fn clear_menu(&mut self, commands: &mut Commands) {
        for item in self.contents.iter() {
            if let Some(entity) = commands.get_entity(*item) {
                entity.despawn_recursive();
            }
        }
    }

    pub fn set_row_text(
        &mut self,
        commands: &mut Commands,
        input: &str,
        row: usize,
        tint: Option<TintOverride>,
    ) {
        let entity = commands.spawn_empty().id();
        commands.add(SpawnText {
            text: AsciiText {
                text: input.to_string(),
                line_length: MENU_SIZE_X - 2,
            },
            tint,
            entity: Some(entity),
            position: Vec3::new(
                (GRID_SIZE_X - MENU_SIZE_X) as f32 / 2.0 * TILE_SIZE + TILE_SIZE * 0.5,
                (GRID_SIZE_Y + MENU_SIZE_Y) as f32 / 2.0 * TILE_SIZE
                    - TILE_SIZE * 0.5
                    - row as f32 * TILE_SIZE,
                900.0,
            ),
        });
        self.contents.push(entity);
    }
}

#[derive(Component)]
pub struct MenuItem;

#[derive(Component)]
pub struct MenuBackground;

fn lock_to_menu(mut menu_background: Query<&mut TextureAtlasSprite, With<MenuBackground>>) {
    for mut sprite in &mut menu_background {
        sprite.custom_size = Some(Vec2::new(
            TILE_SIZE * MENU_SIZE_X as f32,
            TILE_SIZE * MENU_SIZE_Y as f32,
        ));
    }
}

#[derive(Event)]
pub struct OpenMenu(pub Entity);

#[derive(Event)]
pub struct MenuRedraw;

#[derive(Event)]
pub struct CloseMenu;

pub struct CentralMenuPlugin;

impl Plugin for CentralMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenMenu>()
            .add_event::<CloseMenu>()
            .add_event::<MenuRedraw>()
            .init_resource::<CentralMenu>()
            .add_systems(Update, (open_menu, close_menu).chain())
            // FIXME this is schedule abuse, create a schedule or insert a flush correctly
            .add_systems(SpawnScene, (lock_to_menu).chain());
    }
}

pub fn menu_is_open() -> impl Condition<()> {
    IntoSystem::into_system(|menu: Res<CentralMenu>| menu.open)
}

fn open_menu(
    mut commands: Commands,
    mut events: EventReader<OpenMenu>,
    mut open_event: EventWriter<MenuRedraw>,
    mut menu: ResMut<CentralMenu>,
) {
    for event in events.read() {
        if menu.open {
            error!("Central Menu is already open!")
        }
        menu.open = true;
        menu.owner = Some(event.0);

        commands.spawn((
            MenuItem,
            MenuBackground,
            SpatialBundle::from_transform(Transform::from_xyz(
                (GRID_SIZE_X) as f32 / 2.0 * TILE_SIZE,
                (GRID_SIZE_Y) as f32 / 2.0 * TILE_SIZE,
                899.0,
            )),
            GameSprite::MenuBackground,
        ));
        open_event.send(MenuRedraw);
    }
}

// FIXME Probably 1 frame gap where new entities can be spawned as this one despawns
fn close_menu(
    mut commands: Commands,
    menu_items: Query<Entity, With<MenuItem>>,
    mut events: EventReader<CloseMenu>,
    mut menu: ResMut<CentralMenu>,
) {
    for _event in events.read() {
        for entity in &menu_items {
            commands.entity(entity).despawn_recursive();
        }
        if !menu.open {
            error!("Central Menu is already closed!")
        }
        menu.open = false;
        menu.owner = None;
        menu.clear_menu(&mut commands);
    }
}
