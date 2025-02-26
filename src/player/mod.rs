mod components;
mod resource;
mod systems;
mod projectile;
mod upgrades;

use bevy::prelude::*;
use crate::player::projectile::ProjectilePlugin;
use crate::player::systems::*;
use crate::player::upgrades::UpgradePlugin;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_player)
            .add_systems(Update, update_player)
            .add_plugins(ProjectilePlugin)
            .add_plugins(UpgradePlugin);
    }
}

