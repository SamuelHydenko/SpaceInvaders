use bevy::prelude::*;

pub fn setup_scene(
    mut commands: Commands,
){
    commands.spawn(Camera2d{..default()});
}