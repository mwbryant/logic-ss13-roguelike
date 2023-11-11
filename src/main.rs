#![allow(clippy::type_complexity)]
mod grid;
pub mod map;
mod player;

use bevy::prelude::KeyCode::{A, D, P, S, W, X};
use bevy::prelude::*;
use bevy_turborand::prelude::RngPlugin;
use bevy_turborand::{DelegatedRng, GlobalRng, RngComponent};
use grid::{Grid, GridLocation, GridPlugin, LockToGrid};
use map::{setup, update_sprites, GameSprite, Impassable};
use player::{move_player, Player, PlayerTookTurn};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).build())
        .add_plugins(RngPlugin::default().with_rng_seed(0))
        .insert_resource(ClearColor(Color::rgb(0.000001, 0.000001, 0.000001)))
        .add_plugins(GridPlugin::<Impassable>::default())
        .add_systems(Startup, (spawn_player, setup))
        .add_event::<PlayerTookTurn>()
        .add_systems(
            Update,
            (print_debug, update_active_hand, update_sprites, move_player),
        )
        .add_systems(
            Update,
            (npc_wander)
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

    pub fn human_hands() -> Self {
        Self {
            hands: vec![Hand::default(), Hand::default()],
            active: Some(0),
        }
    }
}

#[derive(Debug, Default)]
pub struct Hand {
    holding: Option<Entity>,
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

fn print_debug(player: Query<&Hands>, keyboard: Res<Input<KeyCode>>) {
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
}

fn update_active_hand(mut player: Query<&mut Hands>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(X) {
        let Ok(mut hands) = player.get_single_mut() else {
            return;
        };
        hands.swap_active();
    }
}
