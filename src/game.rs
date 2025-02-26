use bevy::prelude::*;
use crate::alien::AlienPlugin;
use crate::player::PlayerPlugin;
use crate::resolution::ResolutionPlugin;
use crate::systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_scene)
            .add_plugins((
                AlienPlugin,
                ResolutionPlugin,
                PlayerPlugin
            ))
        ;
    }
}