use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Component)]
pub struct EffectTimer(pub Timer);

/// System to despawn particle effects after their timer completes
pub fn cleanup_effects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut EffectTimer)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}