#![allow(clippy::type_complexity)]
pub mod grid;
pub mod interactable;
pub mod map;
mod menu;
pub mod player;
pub mod wfc;

use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::KeyCode::{P, X};
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_turborand::prelude::RngPlugin;
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use grid::{Grid, GridLocation, GridPlugin, LockToGrid, GRID_SIZE_X, GRID_SIZE_Y};
use interactable::{
    player_interact, update_vending_machine_menu_graphics, vending_machine_menu, Interactable,
    VendingMachine,
};
use map::{setup, update_sprites, GameSprite, Impassable};
use menu::{menu_is_open, CentralMenuPlugin, MenuRedraw};
use player::{move_player, Player, PlayerInteract, PlayerTookTurn};
use wfc::{wfc, WfcSettings};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).build())
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
        .init_resource::<WfcSettings>()
        .register_type::<WfcSettings>()
        .add_systems(Startup, (spawn_player, setup))
        .add_event::<PlayerTookTurn>()
        .add_event::<PlayerInteract>()
        .add_systems(PostUpdate, (update_sprites,))
        .add_systems(
            Update,
            (
                print_debug,
                update_active_hand,
                move_player.run_if(not(menu_is_open())),
                vending_machine_menu.run_if(menu_is_open()),
                update_vending_machine_menu_graphics.run_if(on_event::<MenuRedraw>()),
                wfc,
            ),
        )
        .add_systems(
            Update,
            (npc_wander, player_interact)
                .run_if(on_event::<PlayerTookTurn>())
                .before(move_player),
        )
        .run();
}

#[derive(Component, Debug, Default)]
pub struct Hands {
    hands: Vec<Hand>,
    active: Option<usize>,
}

#[derive(Component, Debug, Default)]
pub struct Floor;

impl Hands {
    pub fn swap_active(&mut self) {
        self.active = self.active.map(|index| (index + 1) % self.hands.len());
    }

    pub fn get_active(&self) -> Option<&Hand> {
        self.active.map(|index| &self.hands[index])
    }

    pub fn pickup(&mut self, entity: Entity) -> bool {
        self.active
            .and_then(|idx| self.hands.get_mut(idx))
            .filter(|hand| hand.holding.is_none())
            .map(|hand| {
                hand.holding = Some(entity);
                true
            })
            .unwrap_or(false)
    }

    pub fn can_pickup(&self) -> bool {
        self.active
            .and_then(|idx| self.hands.get(idx))
            .map(|hand| hand.holding.is_none())
            .unwrap_or(false)
    }

    pub fn human_hands() -> Self {
        Self {
            hands: vec![Hand::default(), Hand::default()],
            active: Some(0),
        }
    }
}

#[derive(Debug, Default)]
pub struct Hand {
    pub holding: Option<Entity>,
}

#[derive(Component)]
pub enum Tool {
    Screwdriver,
}

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
    commands.spawn((
        GridLocation::new(1, 3),
        LockToGrid,
        Interactable::VendingMachine,
        VendingMachine::default(),
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
}

fn update_active_hand(mut player: Query<&mut Hands, With<Player>>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(X) {
        let Ok(mut hands) = player.get_single_mut() else {
            return;
        };
        hands.swap_active();
    }
}
