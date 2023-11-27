use bevy::prelude::*;

use crate::log::AddToLog;

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
            commands.add(AddToLog("Used Lighter".to_string(), None));
            lighter.active = !lighter.active;
        }
    }
}
