use bevy::prelude::*;

pub mod alien;
pub mod control;
pub mod game;
pub mod player;
pub mod projectile;
pub mod resolution;

#[cfg(any(feature = "wayland", feature = "x11"))]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: String::from("Space Invaders"),
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        resolution: Vec2::new(512.0, 512.0).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    mode: AssetMode::Processed,
                    file_path: "assets".to_string(),
                    processed_file_path: "assets_imported".to_string(),
                    ..Default::default()
                }),
            game::GamePlugin,
        ))
        .run();
}

#[cfg(not(any(feature = "wayland", feature = "x11")))]
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set((
                RenderPlugin {
                    render_creation: WgpuSettings {
                        backends: None,
                        ..default()
                    }
                    .into(),
                    ..default()
                },
                game::GamePlugin,
            )),
        )
        .run();
}
