use bevy::prelude::*;

#[derive(Resource)]
pub struct AlienManager{
    pub direction: f32,
    pub shift_down : bool,
    pub distance_from_boundary : f32,
    pub reset : bool,
}