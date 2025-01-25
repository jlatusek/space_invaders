use bevy::prelude::*;

use crate::{alien, resolution};

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_projectiles, update_alien_interactions));
    }
}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
}

fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Projectile, &mut Transform)>,
    time: Res<Time>,
    resolution: Res<resolution::Resolution>,
) {
    for (entity, projectile, mut transform) in &mut projectile_query {
        transform.translation.y += projectile.speed * time.delta_secs();
        if transform.translation.y > resolution.screen_dimensions.y * 0.5 {
            commands.entity(entity).despawn();
        }
    }
}

const BULLET_RADIUS: f32 = 24.;

fn update_alien_interactions(
    mut commands: Commands,
    mut alien_query: Query<(&mut alien::Alien, &Transform), Without<alien::Dead>>,
    mut projectile_query: Query<(Entity, &Transform), With<Projectile>>,
) {
    for (mut alien, alien_transform) in &mut alien_query {
        for (projectile_entity, projectile_transform) in &mut projectile_query {
            let projectile_position = projectile_transform.translation.truncate();
            let alien_position = alien_transform.translation.truncate();
            if projectile_position.distance(alien_position) < BULLET_RADIUS {
                alien.dead = true;
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}
