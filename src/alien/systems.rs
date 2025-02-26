use bevy::prelude::*;
use crate::alien::components::*;
use crate::alien::resources::*;
use crate::resolution::Resolution;

const WIDTH: i32 = 10;
const HEIGHT: i32 = 5;
const SPACING: f32 = 32.0;
const SPEED: f32 = 100.;
const SHIFT_AMOUNT: f32 = 32.;


pub fn setup_aliens(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution : Res<Resolution>,
) {
    commands.insert_resource(AlienManager{
        reset : false,
        distance_from_boundary : 0.,
        shift_down : false,
        direction : 1. ,
    });

    let sprite = asset_server.load("alien.png");

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.0)
                - (Vec3::X * WIDTH as f32 * (SPACING * resolution.pixel_ratio) * 0.5)
                - (Vec3::Y * HEIGHT as f32 * (SPACING * resolution.pixel_ratio) * 1.0 )
                + (Vec3::Y * resolution.screen_dimensions.y * 0.5);

            commands.spawn((
                Sprite {
                    image: sprite.clone()   ,
                    ..default()
                },
                Alien{
                    original_position : position,
                    dead : false,
                }
            ))
                .insert(
                    Transform::from_translation(position).with_scale(Vec3::splat(resolution.pixel_ratio))
                );
        }
    }
}

pub fn alien_update(
    mut commands : Commands,
    mut alien_query : Query<(Entity, &mut Transform, &mut Alien, &mut Visibility), Without<Dead>>,
    mut alien_manager: ResMut <AlienManager>,
    resolution : Res<Resolution>,
    time : Res<Time>,
){
    for(entity, mut transform, alien, mut visibility) in alien_query.iter_mut(){
        transform.translation.x += time.delta_secs() * alien_manager.direction * SPEED;
        if transform.translation.x.abs() > resolution.screen_dimensions.x * 0.5{
            alien_manager.shift_down = true;
            alien_manager.distance_from_boundary = resolution.screen_dimensions.x * alien_manager.direction * 0.5 - transform.translation.x;
        }

        if alien.dead{
            commands.entity(entity).insert(Dead{});
            *visibility = Visibility::Hidden;
        }
        else{
            *visibility = Visibility::Visible;
        }

        if transform.translation.y < -resolution.screen_dimensions.y * 0.5{
            alien_manager.reset = true;
        }
    }
}

pub fn alien_logic(
    mut commands : Commands,
    mut alien_query : Query<(Entity, &mut Transform, &mut Alien)>,
    mut alien_manager: ResMut<AlienManager>,
){
    if alien_manager.shift_down{
        alien_manager.shift_down = false;
        alien_manager.direction *= -1.0;
        for(entity, mut transform, mut alien) in alien_query.iter_mut(){
            transform.translation.x += alien_manager.distance_from_boundary;
            transform.translation.y -= SHIFT_AMOUNT;
        }
    }
    if alien_manager.reset{
        alien_manager.reset = false;
        alien_manager.direction *= 1.0;
        for(entity, mut transform, mut alien) in alien_query.iter_mut(){
            transform.translation = alien.original_position;
            if alien.dead{
                alien.dead = false;
                commands.entity(entity).remove::<Dead>();
            }
        }
    }
}