use crate::alien::components::Dead;
use crate::alien::components::Alien;
use bevy::math::primitives::*;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
pub use bevy_hanabi::{ParticleEffect, ParticleEffectBundle};
use crate::particles::components::EffectTimer;
use crate::particles::systems::ParticleEffectHandle;
use crate::player::components::ProjectileProperties;
use crate::player::projectile::components::Projectile;
use crate::resolution::Resolution;
const BULLET_RADIUS: f32 = 24.0;

#[derive(Component)]
pub struct GlowEffect {
    intensity: f32,
    pulse_speed: f32,
    phase: f32,
}

#[derive(Component)]
pub struct GlowEntity(pub Entity);

pub fn spawn_projectile(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    spawn_position: Vec3,
    properties: &ProjectileProperties,
    resolution: &Res<Resolution>,
){
   let bullet_texture = asset_server.load("bullet.png");
    let projectile_entity = commands.spawn((
        Sprite{
            image: bullet_texture,
            ..default()
        },
        Projectile{
            speed: properties.speed,
            damage: properties.damage,
            size: properties.size,
        },
        Transform::from_translation(spawn_position)
            .with_scale(Vec3::splat(resolution.pixel_ratio * properties.size * 1.5))
        )).id();

    add_glow_to_projectile(commands, meshes, materials, projectile_entity, 1.0);
}

pub fn add_glow_to_projectile(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    projectile_entity: Entity,
    size: f32,
) {
    // Create a glow circle mesh
    let rectangle_size = Vec2::new((size + 0.5) * 2.0 , (size + 2.0) * 2.0); // Twice the size of the projectile
    let rectangle_mesh = meshes.add(Mesh::from(Rectangle::new(rectangle_size.x, rectangle_size.y)));

    // Choose a glow color
    let glow_material = materials.add(ColorMaterial::from(Color::srgba(0., 0.5, 0., 0.2)));

    // Spawn the glow effect as a child of the projectile
    let glow_entity = commands
        .spawn((
            Mesh2d(rectangle_mesh),
            MeshMaterial2d(glow_material),
            Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)).with_scale(Vec3::splat(1.0)),
            GlowEffect {
                intensity: 0.7,
                pulse_speed: 5.0,
                phase: 0.0,
            },
        ))
        .id();

    commands.entity(glow_entity).set_parent(projectile_entity);
}

pub fn update_projectiles(
    mut commands: Commands,
    mut projectile_query: Query<(&mut Transform, &Projectile, Entity)>,
    time: Res<Time>,
    resolution: Res<Resolution>,
){
    for(mut transform, projectile, entity) in projectile_query.iter_mut(){
        transform.translation.y += projectile.speed * time.delta_secs();
        if transform.translation.y > resolution.screen_dimensions.y * 0.5{
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn enemy_interaction(
    mut commands: Commands,
    mut enemy_query: Query<(&mut Alien, &Transform), Without<Dead>>,
    mut projectile_query: Query<(&Transform, Entity, Option<&GlowEntity>), With<Projectile>>,
    effect_handle: Res<ParticleEffectHandle>, // Access the particle effect handle
) {
    for (mut alien, alien_transform) in enemy_query.iter_mut() {
        for (projectile_transform, projectile, glow_entity) in projectile_query.iter_mut() {
            let projectile_pos = Vec2::new(
                projectile_transform.translation.x,
                projectile_transform.translation.y,
            );
            let alien_pos = Vec2::new(
                alien_transform.translation.x,
                alien_transform.translation.y,
            );

            if Vec2::distance(alien_pos, projectile_pos) < BULLET_RADIUS {
                // Mark the alien as dead
                alien.dead = true;

                // Despawn the projectile
                commands.entity(projectile).despawn_recursive();
                // Spawn the particle effect at the alien's position
                commands.spawn((
                    ParticleEffectBundle {
                        effect: ParticleEffect::new(effect_handle.0.clone()), // Wrap the handle
                        transform: Transform::from_translation(alien_transform.translation),
                        ..Default::default()
                    },
                    Name::new("ExplosionEffect"),
                    EffectTimer(Timer::from_seconds(1.0, TimerMode::Once)), // Add a 2-second timer
                ));
            }
        }
    }
}