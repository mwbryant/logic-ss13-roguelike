use bevy::prelude::*;

use crate::GameState;

#[derive(Resource, Default)]
pub struct CentralMenu {
    pub open: bool,
    pub owner: Option<Entity>,
}

#[derive(Event)]
pub struct OpenMenu(Entity);

#[derive(Event)]
pub struct CloseMenu;

pub struct CentralMenuPlugin;

impl Plugin for CentralMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<OpenMenu>()
            .add_event::<CloseMenu>()
            .init_resource::<CentralMenu>()
            .add_systems(Update, (open_menu, close_menu, update_game_state).chain());
    }
}

fn open_menu(mut events: EventReader<OpenMenu>, mut menu: ResMut<CentralMenu>) {
    for event in events.read() {
        if menu.open {
            error!("Central Menu is already open!")
        }
        menu.open = true;
        menu.owner = Some(event.0);
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

fn update_game_state(menu: Res<CentralMenu>, mut game_state: ResMut<GameState>) {
    game_state.in_menu = menu.open;
}
