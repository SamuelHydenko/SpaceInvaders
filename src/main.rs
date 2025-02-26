mod game;
mod systems;
mod alien;
mod resolution;
mod player;
mod particles;

use bevy::prelude::*;
use crate::game::GamePlugin;
use bevy_hanabi::prelude::*;

fn main() {
    App::new()
        //default plugin
        .add_plugins(DefaultPlugins.set(WindowPlugin{
            primary_window: Some(Window{
                title : String::from("Space Invaders"),
                position : WindowPosition::Centered(MonitorSelection::Primary),
                resolution : Vec2::new(1024., 800.).into(),
                ..default()
                }),
            ..default()
            })
            .set(ImagePlugin::default_nearest()),
        )
        //list of plugins
        .add_plugins(
            (GamePlugin, HanabiPlugin)
        )
        .run();
}
