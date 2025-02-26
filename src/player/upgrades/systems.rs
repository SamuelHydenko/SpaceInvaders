use bevy::prelude::*;
use rand::random;
use crate::alien::components::*;
use crate::player::components::Player;
use crate::player::upgrades::components::{Upgrade, UpgradeType};
use crate::resolution::Resolution;

const UPGRADE_SPAWN_CHANCE : f32 = 100.;
const UPGRADE_SPEED : f32 = 50.;
//spawns upgrade upon enemies death
pub fn upgrade_spawning(
    mut commands: Commands,
    mut enemy_query: Query<(&Alien, &Transform), (With<Dead>, Changed<Dead>)>,
    resolution: Res<Resolution>,
    asset_server: Res<AssetServer>,
) {
    let upgrade_texture = asset_server.load("upgrade.png");
    for (mut alien, enemy_transform) in enemy_query.iter_mut() {
        if random::<f32>() * 100. <= UPGRADE_SPAWN_CHANCE {
            let upgrade_type = match random::<u8>() % 4 {
                0 => UpgradeType::ProjectileSpeed(1.2),
                1 => UpgradeType::ProjectileDamage(0.5),
                2 => UpgradeType::FireRate(1.2),
                _ => UpgradeType::ProjectileSize(1.1),
            };

            commands.spawn((
                Sprite {
                    image: upgrade_texture.clone(),
                    ..default()
                },
                Upgrade {
                    upgrade_type,
                },
                Transform::from_translation(enemy_transform.translation)
                    .with_scale(Vec3::splat(resolution.pixel_ratio)),
            ));
        }
    }
}
//responsible for moving upgrades to the bottom
pub fn upgrade_move(
    mut commands: Commands,
    mut upgrade_query: Query<(Entity, &mut Transform), With<Upgrade>>,
    resolution: Res<Resolution>,
    time: Res<Time>,
){
    for(entity, mut transform) in upgrade_query.iter_mut(){
        transform.translation.y -= UPGRADE_SPEED * time.delta_secs();

        if transform.translation.y <= -resolution.screen_dimensions.y * 0.5{
            commands.entity(entity).despawn();
        }
    }
}
//responsible for interaction with player
pub fn upgrade_interaction(
    mut commands: Commands,
    mut player_query: Query<(&mut Player, &Transform)>,
    upgrade_query: Query<(Entity, &Transform, &Upgrade), With<Upgrade>>,
){
    let Ok((mut player, player_transform)) = player_query.get_single_mut() else {
        return;
    };

    for (upgrade_entity, mut upgrade_transform, upgrade) in upgrade_query.iter(){
        let distance = player_transform.translation.distance(upgrade_transform.translation);

        if distance <= 30. {
            match upgrade.upgrade_type{
                UpgradeType::ProjectileSpeed(multiplier) => {
                    player.projectile_properties.speed *= multiplier;
                },
                UpgradeType::ProjectileDamage(additional) => {
                    player.projectile_properties.speed += additional;
                },
                UpgradeType::FireRate(multiplier) => {
                    player.projectile_properties.speed *= 1.0/multiplier;
                },
                UpgradeType::ProjectileSize(multiplier) => {
                    player.projectile_properties.speed *= multiplier;
                },
            }
            commands.entity(upgrade_entity).despawn();
        }
    }
}