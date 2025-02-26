pub mod systems;
pub mod resources;
pub mod components;
use crate::player::projectile::systems::{add_glow_to_projectile, enemy_interaction, update_projectiles};
use crate::particles::systems::*;
use bevy::prelude::*;
use crate::particles::components::cleanup_effects;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_particles)
            .add_systems(Update, (
            update_projectiles,
            enemy_interaction,
            cleanup_effects
        ));
    }
}