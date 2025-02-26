use bevy::prelude::*;

#[derive(Component)]
pub enum UpgradeType {
    ProjectileSpeed(f32),    // Multiplier for speed
    ProjectileDamage(f32),   // Additional damage
    FireRate(f32),          // Multiplier for cooldown reduction
    ProjectileSize(f32),    // Multiplier for size
}

#[derive(Component)]
pub struct Upgrade {
    pub upgrade_type: UpgradeType,
}
