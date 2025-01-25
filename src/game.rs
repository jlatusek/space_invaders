use bevy::prelude::*;

use crate::alien;
use crate::resolution;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((resolution::ResolutionPlugin, alien::AlienPlugin))
            .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d {
        ..Default::default()
    });
}