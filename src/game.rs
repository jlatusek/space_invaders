use bevy::prelude::*;

use crate::alien;
use crate::control;
use crate::player;
use crate::projectile;
use crate::resolution;
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            resolution::ResolutionPlugin,
            alien::AlienPlugin,
            projectile::ProjectilePlugin,
            player::PlayerPlugin,
            control::ControlPlugin,
        ))
        .add_systems(PreStartup, load_assets)
        .add_systems(Startup, setup_scene);
    }
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2d {
        ..Default::default()
    });
}

#[derive(Resource, Default)]
pub struct SIAssetsSource {
    pub player_image: Handle<Image>,
    pub bullet_image: Handle<Image>,
}

fn load_assets(asset_server: Res<AssetServer>, mut commands: Commands) {
    let assets = SIAssetsSource {
        player_image: asset_server.load("player.png"),
        bullet_image: asset_server.load("bullet.png"),
    };
    commands.insert_resource(assets);
}
