use bevy::prelude::*;

#[derive(Component)]
pub struct Alien {
    pub dead: bool,
    pub original_position: Vec3,
}
#[derive(Component)]
pub struct Dead{}
