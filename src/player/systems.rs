use bevy::prelude::*;
use crate::player::components::*;
use crate::player::projectile::components::*;
use crate::player::projectile::systems::spawn_projectile;
use crate::resolution::Resolution;


const PLAYER_SPEED: f32 = 200.0;
const SHOT_COOLDOWN: f32 = 0.5;

pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    resolution : Res<Resolution>,
){
    let player_texture = asset_server.load("player.png");

    commands.spawn((
        Sprite{
            image : player_texture,
            ..default()
        },
        Player{
            shoot_timer : 0.,
            projectile_properties : ProjectileProperties::default(),
        }
    ))
        .insert(Transform::from_xyz(0., -(resolution.screen_dimensions.y*0.5) + (resolution.pixel_ratio*5.0), 0.).with_scale(Vec3::splat(resolution.pixel_ratio)));
}

pub fn update_player(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    resolution: Res<Resolution>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let (mut player_transform, mut player) = player_query.single_mut();

    let mut horizontal = 0.0;

    if keys.pressed(KeyCode::ArrowRight){
        horizontal += 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft){
        horizontal -= 1.0;
    }
    player_transform.translation.x += horizontal * time.delta_secs() * PLAYER_SPEED;

    let left_bound = -resolution.screen_dimensions.x * 0.5 + 16.;
    let right_bound = resolution.screen_dimensions.x * 0.5 - 16.;

    if player_transform.translation.x < left_bound {
        player_transform.translation.x = left_bound;
    }
    if player_transform.translation.x > right_bound {
        player_transform.translation.x = right_bound;
    }

    player.shoot_timer -= time.delta_secs();

    if keys.pressed(KeyCode::Space) && player.shoot_timer <= 0.0 {
        player.shoot_timer = player.projectile_properties.cooldown;

        spawn_projectile(
            &mut commands,
            &asset_server,
            &mut meshes,
            &mut materials,
            player_transform.translation,
            &player.projectile_properties,
            &resolution,
        );
    }
}