pub mod components;
pub mod resources;
pub mod systems;
use crate::player::upgrades::systems::*;
use bevy::prelude::*;

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, upgrade_interaction).
            add_systems(Update, upgrade_spawning).
            add_systems(Update, upgrade_move);
    }
}