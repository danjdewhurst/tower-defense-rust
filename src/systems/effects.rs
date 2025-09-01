use crate::components::*;
use bevy::prelude::*;

pub fn update_effects(
    mut hit_effect_query: Query<(Entity, &mut HitEffect, &mut Transform)>,
    mut explosion_query: Query<
        (Entity, &mut ExplosionParticle, &mut Transform),
        Without<HitEffect>,
    >,
    mut commands: Commands,
    time: Res<Time>,
) {
    // Handle hit effects
    for (entity, mut effect, mut transform) in hit_effect_query.iter_mut() {
        effect.timer.tick(time.delta());

        // Fade out effect
        let alpha = 1.0 - effect.timer.fraction();
        transform.scale = Vec3::splat(1.0 + (1.0 - alpha) * 2.0);

        if effect.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }

    // Handle explosion particles
    for (entity, mut particle, mut transform) in explosion_query.iter_mut() {
        particle.timer.tick(time.delta());

        // Move particle
        transform.translation += Vec3::new(
            particle.velocity.x * time.delta_secs(),
            particle.velocity.y * time.delta_secs(),
            0.0,
        );

        // Fade and shrink
        let alpha = 1.0 - particle.timer.fraction();
        transform.scale = Vec3::splat(alpha);

        // Slow down particle
        particle.velocity *= 0.95;

        if particle.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}
