use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    pub damage: f32,
    pub size: f32,
}