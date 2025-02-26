use bevy::prelude::*;
use bevy_hanabi::prelude::*;
#[derive(Resource)]
pub struct ParticleEffectHandle(pub Handle<EffectAsset>);

pub fn setup_particles(mut commands: Commands, mut effects: ResMut<Assets<EffectAsset>>) {
    let mut gradient = Gradient::new();
    gradient.add_key(0.0, Vec4::new(0.0, 1.0, 0.0, 0.7));  // Bright yellow at the start
    gradient.add_key(0.5, Vec4::new(0.0, 1.0, 0.0, 0.3));  // Orange in the middle
    gradient.add_key(1.0, Vec4::new(0.0, 1.0, 0.0, 0.0));  // Fully transparent at the end

    let mut module = Module::default();

    // Initialize particle position in a spherical region (simulating explosion)
    let init_pos = SetPositionSphereModifier {
        center: module.lit(Vec3::ZERO),  // Explosion center
        radius: module.lit(5.),  // Small radius for burst
        dimension: ShapeDimension::Surface,  // Eject particles from surface
    };

    // Initial velocity, making particles move outward (randomized directions)
    let init_vel = SetVelocitySphereModifier {
        center: module.lit(Vec3::ZERO),  // Explosion center
        speed: module.lit(10.0),  // High speed for explosion
    };

    // Set random lifetime for each particle, for a quick burst
    let lifetime = module.lit(1.0);  // Short lifetime for explosive particles
    let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

    // Random size (particles start bigger and shrink over time)
    let start_size = module.lit(2.0);  // Bigger particles at start
    let end_size = module.lit(0.1);  // Smaller particles at the end of their lifetime
    let size_over_lifetime = SetAttributeModifier::new(Attribute::SIZE, start_size);

    // Gravity or downward acceleration to simulate gravity pulling particles
    let accel = module.lit(Vec3::new(0., -5., 0.));  // Apply gravity downward
    let update_accel = AccelModifier::new(accel);

    // Create the explosion effect asset
    let effect = EffectAsset::new(5000, Spawner::rate(100.0.into()), module)  // Many particles per second
        .with_name("ExplosionEffect")
        .init(init_pos)
        .init(init_vel)
        .init(init_lifetime)
        .init(size_over_lifetime)
        .update(update_accel)
        .render(ColorOverLifetimeModifier { gradient });

    // Add the effect to the asset system
    let effect_handle = effects.add(effect);

    // Insert the effect handle as a resource
    commands.insert_resource(ParticleEffectHandle(effect_handle));
}