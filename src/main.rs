mod grid;
pub mod map;
mod player;

use bevy::prelude::KeyCode::{A, D, P, S, W, X};
use bevy::prelude::*;
use grid::{GridLocation, GridPlugin, LockToGrid};
use map::{setup, update_sprites, GameSprite, Impassable};
use player::{move_player, Player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).build())
        .insert_resource(ClearColor(Color::rgb(0.000001, 0.000001, 0.000001)))
        .add_plugins(GridPlugin::<Impassable>::default())
        .add_systems(Startup, (spawn_player, setup))
        .add_systems(
            Update,
            (print_debug, update_active_hand, update_sprites, move_player),
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

fn print_debug(player: Query<&Hands>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(P) {
        info!("{:?}", player.get_single());
    }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Hands::human_hands(),
        GridLocation::new(0, 0),
        LockToGrid,
        Impassable,
        GameSprite::Player,
        Player,
        SpatialBundle::default(),
    ));
    commands.spawn((
        Hands::human_hands(),
        GridLocation::new(1, 0),
        LockToGrid,
        Impassable,
        GameSprite::Npc,
        SpatialBundle::default(),
    ));
}

fn update_active_hand(mut player: Query<&mut Hands>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(X) {
        let Ok(mut hands) = player.get_single_mut() else {
            return;
        };
        hands.swap_active();
    }
}
