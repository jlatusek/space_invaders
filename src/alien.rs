use bevy::prelude::*;

use crate::resolution;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens).add_systems(
            Update,
            update_aliens, //, manage_alien_logic),
        );
    }
}

#[derive(Component)]
pub struct Alien {
    pub dead: bool,
    pub original_position: Vec3,
}

#[derive(Component)]
pub struct Dead;

#[derive(Resource)]
pub struct AlienManager {
    pub direction: f32,
    pub shift_aliens_down: bool,
    pub dist_from_boundary: f32,
    pub reset: bool,
}

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 24.;
const SPEED: f32 = 100.0;
const ALIEN_SHIFT_AMOUNT: f32 = 32.;

fn setup_aliens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution: Res<resolution::Resolution>,
) {
    commands.insert_resource(AlienManager {
        reset: false,
        dist_from_boundary: 0.,
        shift_aliens_down: false,
        direction: 1.,
    });
    let alien_texture: Handle<Image> = asset_server.load("alien.png");
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.)
                -(Vec3::X * WIDTH as f32 * SPACING * 0.5) //Center alien on the x axis
                -(Vec3::Y * HEIGHT as f32 * SPACING * 1.0 ) // Displace the aliens below the x axis so that we can displace them to the top the screen in the next line
                +(Vec3::Y * resolution.screen_dimensions.y * 0.5); // Displace the aliens to the top of the screen
            commands.spawn((
                Sprite::from_image(alien_texture.clone()),
                Transform::from_translation(position)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
                Alien {
                    original_position: position,
                    dead: false,
                },
            ));
        }
    }
}

fn update_aliens(
    mut commands: Commands,
    mut alien_query: Query<(Entity, &Alien, &mut Transform, &mut Visibility), Without<Dead>>,
    mut alien_manager: ResMut<AlienManager>,
    resolution: Res<resolution::Resolution>,
    time: Res<Time>,
) {
    for (entity, alien, mut transform, mut visibility) in alien_query.iter_mut() {
        transform.translation.x += time.delta_secs() * alien_manager.direction * SPEED;
        if transform.translation.x.abs() > resolution.screen_dimensions.x * 0.5 {
            alien_manager.shift_aliens_down = true;
            alien_manager.dist_from_boundary =
                resolution.screen_dimensions.x * alien_manager.direction * 0.5
                    - transform.translation.x;
            //calculates the delta x we need to move the alien to get it back into our bounds
        }
        if alien.dead {
            commands.entity(entity).insert(Dead {});
            *visibility = Visibility::Hidden;
        } else {
            *visibility = Visibility::Visible;
        }
        //if the aliens have made it out of the bottom of the screen we have lost the game and should reset
        if transform.translation.y < -resolution.screen_dimensions.y * 0.5 {
            alien_manager.reset = true;
        }
    }
}
