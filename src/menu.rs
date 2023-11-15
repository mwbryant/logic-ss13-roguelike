use bevy::prelude::*;

use crate::{
    grid::{GRID_SIZE_X, GRID_SIZE_Y, TILE_SIZE},
    map::GameSprite,
};

pub const MENU_SIZE_X: usize = 48;
pub const MENU_SIZE_Y: usize = 24;

#[derive(Resource)]
pub struct CentralMenu {
    pub open: bool,
    pub contents: [[Option<Entity>; MENU_SIZE_Y]; MENU_SIZE_X],
    pub owner: Option<Entity>,
}

// https://github.com/rust-lang/rust/issues/44796#issuecomment-967747810
const INIT: Option<Entity> = None;
const INIT_INNER: [Option<Entity>; MENU_SIZE_Y] = [INIT; MENU_SIZE_Y];

impl Default for CentralMenu {
    fn default() -> Self {
        Self {
            open: false,
            contents: [INIT_INNER; MENU_SIZE_X],
            owner: None,
        }
    }
}

#[derive(Component)]
pub struct MenuItem;

#[derive(Component)]
pub struct MenuBackground;

fn lock_to_menu(mut positions: Query<&mut Transform, With<MenuItem>>, menu: Res<CentralMenu>) {
    let menu_position = Vec2::new(
        (GRID_SIZE_X - MENU_SIZE_X) as f32 / 2.0 * TILE_SIZE + TILE_SIZE * 0.5,
        (GRID_SIZE_Y - MENU_SIZE_Y) as f32 / 2.0 * TILE_SIZE - TILE_SIZE * 0.5,
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
pub struct CloseMenu;

pub struct CentralMenuPlugin;

impl Plugin for CentralMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenMenu>()
            .add_event::<CloseMenu>()
            .init_resource::<CentralMenu>()
            .add_systems(Update, (open_menu, lock_to_menu, close_menu).chain());
    }
}

pub fn menu_is_open() -> impl Condition<()> {
    IntoSystem::into_system(|menu: Res<CentralMenu>| menu.open)
}

fn open_menu(
    mut commands: Commands,
    mut events: EventReader<OpenMenu>,
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
        let wall = commands
            .spawn((MenuItem, SpatialBundle::default(), GameSprite::Wall))
            .id();
        menu.contents[0][1] = Some(wall);
    }
}

fn close_menu(mut events: EventReader<CloseMenu>, mut menu: ResMut<CentralMenu>) {
    for _event in events.read() {
        if !menu.open {
            error!("Central Menu is already closed!")
        }
        menu.open = false;
        menu.owner = None;
    }
}

impl CentralMenu {
    pub fn iter(&self) -> impl Iterator<Item = (Entity, IVec2)> + '_ {
        self.contents
            .iter()
            .flatten()
            .enumerate()
            .filter_map(|(i, cell)| {
                cell.as_ref().map(|&entity| {
                    (
                        entity,
                        IVec2::new(
                            i as i32 / MENU_SIZE_Y as i32,
                            MENU_SIZE_Y as i32 - i as i32 % MENU_SIZE_Y as i32,
                        ),
                    )
                })
            })
    }
}
