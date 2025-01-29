use bevy::prelude::*;

use crate::player;
pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_control_events);
    }
}

fn update_control_events(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_event_writter: EventWriter<player::PlayerEvent>,
) {
    let mut player_event = player::PlayerEvent::default();
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        player_event.move_direction += -1;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        player_event.move_direction += 1;
    }
    if keys.pressed(KeyCode::Space){
        player_event.shoot = true;
    }
    player_event_writter.send(player_event);
}
