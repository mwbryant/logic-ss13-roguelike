use array2d::Array2D;
use bevy::prelude::*;

use crate::{
    grid::{GRID_SIZE_X, GRID_SIZE_Y, TILE_SIZE},
    map::{GameSprite, TintOverride},
};

pub const MENU_SIZE_X: usize = 48;
pub const MENU_SIZE_Y: usize = 24;

#[derive(Resource)]
pub struct CentralMenu {
    pub open: bool,
    pub contents: Array2D<Option<Entity>>,
    pub owner: Option<Entity>,
}

impl Default for CentralMenu {
    fn default() -> Self {
        Self {
            open: false,
            contents: Array2D::filled_with(None, MENU_SIZE_Y, MENU_SIZE_X),
            owner: None,
        }
    }
}

impl CentralMenu {
    pub fn clear_menu(&mut self, commands: &mut Commands) {
        for item in self.contents.elements_row_major_iter().flatten() {
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
        for x in 0..self.contents.num_columns() {
            // ugh
            let entity = input.chars().nth(x).map(|c| {
                let entity = commands
                    .spawn((MenuItem, SpatialBundle::default(), GameSprite::Text(c)))
                    .id();
                if let Some(tint) = &tint {
                    commands.entity(entity).insert(tint.clone());
                }
                entity
            });
            self.contents[(row, x)] = entity;
        }
    }
}

#[derive(Component)]
pub struct MenuItem;

#[derive(Component)]
pub struct MenuBackground;

fn lock_to_menu(
    mut positions: Query<&mut Transform, With<MenuItem>>,
    mut menu_background: Query<&mut TextureAtlasSprite, With<MenuBackground>>,
    menu: Res<CentralMenu>,
) {
    for mut sprite in &mut menu_background {
        sprite.custom_size = Some(Vec2::new(
            TILE_SIZE * MENU_SIZE_X as f32,
            TILE_SIZE * MENU_SIZE_Y as f32,
        ));
    }
    let menu_position = Vec2::new(
        (GRID_SIZE_X - menu.contents.num_columns()) as f32 / 2.0 * TILE_SIZE + TILE_SIZE * 0.5,
        (GRID_SIZE_Y - menu.contents.num_rows()) as f32 / 2.0 * TILE_SIZE - TILE_SIZE * 0.5,
    );
    for (entity, location) in menu.iter() {
        if let Ok(mut position) = positions.get_mut(entity) {
            position.translation.x = menu_position.x + location.x as f32 * TILE_SIZE;
            position.translation.y = menu_position.y + location.y as f32 * TILE_SIZE;
            position.translation.z = 900.0;
        }
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
    }
}

impl CentralMenu {
    pub fn iter(&self) -> impl Iterator<Item = (Entity, IVec2)> + '_ {
        let x = self.contents.num_rows() as i32;
        let y = self.contents.num_columns() as i32;
        self.contents
            .elements_row_major_iter()
            .enumerate()
            .filter_map(move |(i, cell)| {
                cell.as_ref()
                    .map(|&entity| (entity, IVec2::new(i as i32 % y, x - i as i32 / y)))
            })
    }
}
