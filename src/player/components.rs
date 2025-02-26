use bevy::prelude::*;
use crate::player::projectile::components::Projectile;
use crate::player::upgrades::components::Upgrade;

#[derive(Component)]
pub struct Player {
        pub shoot_timer: f32,
        pub projectile_properties: ProjectileProperties,
}

#[derive(Clone)]
pub struct ProjectileProperties {
        pub speed: f32,
        pub damage: f32,
        pub size: f32,
        pub cooldown: f32,
}
impl Default for ProjectileProperties {
        fn default() -> Self {
                Self {
                        speed: 400.0,        // Base projectile speed
                        damage: 1.0,         // Base damage
                        size: 1.0,           // Base size multiplier
                        cooldown: 0.5,       // Base cooldown between shots
                }
        }
}