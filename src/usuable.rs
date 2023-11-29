use bevy::prelude::*;

use crate::{graphics::TintOverride, log::AddToLog, player::PlayerCombined, Cigarette};

#[derive(Event)]
pub struct PlayerUsed(pub Entity);

#[derive(Component)]
pub struct Lighter {
    pub active: bool,
}

pub fn use_lighter(
    mut commands: Commands,
    mut events: EventReader<PlayerUsed>,
    mut lighters: Query<&mut Lighter>,
) {
    for event in events.read() {
        if let Ok(mut lighter) = lighters.get_mut(event.0) {
            lighter.active = !lighter.active;
            if lighter.active {
                commands.add(AddToLog("Activated Lighter".to_string(), None));
                commands
                    .entity(event.0)
                    .insert(TintOverride(Color::ORANGE_RED));
            } else {
                commands.add(AddToLog("Deactivated Lighter".to_string(), None));
                commands.entity(event.0).insert(TintOverride(Color::GREEN));
            }
        }
    }
}

pub fn use_lighter_on_cig(
    mut commands: Commands,
    mut events: EventReader<PlayerCombined>,
    lighters: Query<&Lighter>,
    mut cigs: Query<&mut Cigarette>,
) {
    for event in events.read() {
        if let Ok(lighter) = lighters.get(event.0) {
            if let Ok(mut cig) = cigs.get_mut(event.1) {
                if lighter.active {
                    commands.add(AddToLog("Lit Cigarette".to_string(), None));
                    cig.burning = true;
                    commands.entity(event.1).insert(TintOverride(Color::ORANGE));
                } else {
                    commands.add(AddToLog("The lighter is off!".to_string(), None));
                }
            }
        }
    }
}
