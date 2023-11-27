#![allow(clippy::type_complexity)]
pub mod graphics;
pub mod grid;
mod hands;
pub mod interactable;
pub mod log;
mod menu;
pub mod player;
pub mod status_bar;
mod text;
mod usuable;
pub mod wfc;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::KeyCode::P;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_turborand::prelude::RngPlugin;
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use graphics::{setup, update_sprites, GameSprite, Impassable, TintOverride};
use grid::{Grid, GridLocation, GridPlugin, LockToGrid, GRID_SIZE_X, GRID_SIZE_Y};
use hands::{handle_give_item, GiveItem, Hands};
use interactable::{
    player_interact, update_vending_machine_menu_graphics, vending_machine_menu, Interactable,
    VendingMachine,
};
use log::{lock_to_log, setup_log, Log};
use menu::{menu_is_open, CentralMenuPlugin, MenuRedraw};
use player::{
    move_player, update_active_hand, use_active_hand, Player, PlayerInteract, PlayerTookTurn,
};
use status_bar::{setup_status_bar, StatusBar, UpdateStatusBar};
use usuable::{use_lighter, Lighter, PlayerUsed};
use wfc::{wfc, WfcSettings};

pub const SCREEN_SIZE_X: usize = 85;
pub const SCREEN_SIZE_Y: usize = 48;
pub const TILE_SIZE: f32 = 9.0;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        // present_mode: bevy::window::PresentMode::Mailbox,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_plugins(RngPlugin::default().with_rng_seed(0))
        .insert_resource(ClearColor(Color::rgb(0.000001, 0.000001, 0.000001)))
        .add_plugins((
            GridPlugin::<Floor>::default(),
            GridPlugin::<Impassable>::default(),
            GridPlugin::<Interactable>::default(),
            CentralMenuPlugin,
        ))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .init_resource::<WfcSettings>()
        .register_type::<WfcSettings>()
        .add_systems(Startup, (spawn_player, setup, setup_log, setup_status_bar))
        .add_event::<PlayerTookTurn>()
        .add_event::<PlayerInteract>()
        .add_event::<GiveItem>()
        .add_event::<PlayerUsed>()
        .add_systems(PostUpdate, (update_sprites,))
        .init_resource::<Log>()
        .init_resource::<StatusBar>()
        // please use schedules
        .add_systems(First, vending_machine_menu.run_if(menu_is_open()))
        .add_systems(
            Update,
            (
                print_debug,
                update_active_hand,
                handle_give_item,
                use_lighter,
                move_player.run_if(not(menu_is_open())),
                use_active_hand,
                update_vending_machine_menu_graphics.run_if(on_event::<MenuRedraw>()),
                wfc,
            ),
        )
        // XXX this is schedule abuse
        .add_systems(SpawnScene, lock_to_log)
        .add_systems(
            Update,
            (npc_wander, player_interact)
                .run_if(on_event::<PlayerTookTurn>())
                .before(move_player),
        )
        .run();
}

#[derive(Component, Debug, Default)]
pub struct Floor;

#[derive(Component)]
pub enum Tool {
    Screwdriver,
}

#[derive(Component)]
pub struct Cigarette;

fn npc_wander(
    mut locations: Query<(Entity, &mut GridLocation, &mut RngComponent), With<Npc>>,
    mut grid: ResMut<Grid<Impassable>>,
) {
    for (entity, mut location, mut rng) in &mut locations {
        let mut current = location.get_location();
        let direction = rng.i32(0..=3);
        match direction {
            0 => current += IVec2::new(-1, 0),
            1 => current += IVec2::new(1, 0),
            2 => current += IVec2::new(0, -1),
            3 => current += IVec2::new(0, 1),
            _ => unreachable!(),
        }

        if !grid.occupied(&current.into()) && location.try_set_location(current).is_ok() {
            // PERF
            grid.force_update(entity, &current.into());
        }
    }
}

#[derive(Component)]
pub struct Npc;

fn print_debug(player: Query<&Hands, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(P) {
        info!("{:?}", player.get_single());
    }
}

fn spawn_player(mut commands: Commands, mut global_rng: ResMut<GlobalRng>) {
    commands.spawn((
        Hands::human_hands(),
        GridLocation::new(0, 0),
        LockToGrid,
        RngComponent::from(&mut global_rng),
        Impassable,
        GameSprite::Player,
        Player,
        SpatialBundle::default(),
    ));
    for x in 0..5 {
        commands.spawn((
            Hands::human_hands(),
            GridLocation::new(x + 1, 0),
            Npc,
            LockToGrid,
            RngComponent::from(&mut global_rng),
            Impassable,
            GameSprite::Npc,
            SpatialBundle::default(),
        ));
    }
    let machine = vec![
        commands
            .spawn((
                Tool::Screwdriver,
                Name::new("Screwdriver"),
                LockToGrid,
                GameSprite::Text('s'),
                TintOverride(Color::GREEN),
                SpatialBundle::HIDDEN_IDENTITY,
                Floor,
            ))
            .id(),
        commands
            .spawn((
                Lighter { active: false },
                Name::new("Lighter"),
                LockToGrid,
                GameSprite::Text('l'),
                TintOverride(Color::ORANGE_RED),
                SpatialBundle::HIDDEN_IDENTITY,
                Floor,
            ))
            .id(),
        commands
            .spawn((
                Cigarette,
                Name::new("Cigarette"),
                GameSprite::Text('c'),
                LockToGrid,
                TintOverride(Color::WHITE),
                SpatialBundle::HIDDEN_IDENTITY,
                Floor,
            ))
            .id(),
    ];
    commands.spawn((
        GridLocation::new(1, 3),
        LockToGrid,
        Interactable::VendingMachine,
        VendingMachine {
            selection: 0,
            options: machine,
        },
        Impassable,
        GameSprite::VendingMachine,
        SpatialBundle::default(),
    ));
    commands.spawn_batch((0..GRID_SIZE_X).flat_map(|x| {
        (0..GRID_SIZE_Y).map(move |y| {
            (
                LockToGrid,
                GridLocation::new(x as u32, y as u32),
                Floor,
                GameSprite::Floor,
                SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, -100.0)),
            )
        })
    }));
    commands.add(UpdateStatusBar);
}
