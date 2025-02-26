use bevy::prelude::*;
use crate::alien::systems::*;

pub mod components;
pub mod resources;
pub mod systems;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_aliens)
            .add_systems(Update, (
                alien_update,
                alien_logic
            ));
    }
}