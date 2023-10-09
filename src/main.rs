use bevy::prelude::*;
use bevy::prelude::KeyCode::{P, W, X, A, S, D};
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

fn main() {
    App::new().add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()).build())
        .insert_resource(ClearColor(Color::rgb(0.000001, 0.000001, 0.000001)))
        .add_plugins(PixelCameraPlugin)
        .add_systems(Startup, (spawn_camera, spawn_player))
        .add_systems(Update, (print_debug, update_active_hand, move_camera))
        .run();
}

#[derive(Component, Debug, Default)]
pub struct Hands {
    hands: Vec<Hand>,
    active: Option<usize>
}

impl Hands {
    pub fn swap_active(&mut self){
       self.active =  self.active.map(|index| (index + 1) % self.hands.len());
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
           active: Some(0)
       }
    }
}

#[derive(Debug, Default)]
pub struct Hand {
    holding: Option<Entity>
}

#[derive(Component)]
pub enum Tool {
   Screwdriver
}

fn move_camera(mut camera: Query<&mut Transform, With<Camera>>, keyboard: Res<Input<KeyCode>>) {
   let mut camera = camera.single_mut();
    if keyboard.pressed(W) {
        camera.translation += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard.pressed(S) {
        camera.translation += Vec3::new(0.0, -1.0, 0.0);
    }
    if keyboard.pressed(A) {
        camera.translation += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard.pressed(D) {
        camera.translation += Vec3::new(1.0, 0.0, 0.0);
    }
}

fn print_debug(player: Query<&Hands>, keyboard: Res<Input<KeyCode>>) {
   if keyboard.just_pressed(P) {
       info!("{:?}", player.get_single());
   }
}

fn spawn_player(mut commands: Commands) {
    commands.spawn(Hands::human_hands());
}

fn update_active_hand(mut player: Query<&mut Hands>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(X) {
        let Ok(mut hands) = player.get_single_mut() else { return; };
        hands.swap_active();
    }
}

fn spawn_camera(mut commands: Commands, assets: Res<AssetServer>) {
    let image = assets.load("props.png");
    commands.spawn(SpriteBundle {
        texture: image,
        ..default()
    });
    commands.spawn(PixelCameraBundle::from_resolution(320, 240, false)) ;
}