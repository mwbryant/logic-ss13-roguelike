use bevy::prelude::*;

use crate::{graphics::TintOverride, log::AddToLog};

#[derive(Event)]
pub struct PlayerUsed(pub Entity);

#[derive(Event)]
pub struct PlayerCombined(pub Entity, pub Entity);

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
