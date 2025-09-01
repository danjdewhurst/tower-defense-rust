use super::sound::{create_sound_effect_visual, play_console_beep, SoundType};
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn bullet_movement(
    mut bullet_query: Query<(Entity, &mut Transform, &Bullet)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, bullet) in bullet_query.iter_mut() {
        transform.translation += Vec3::new(bullet.direction.x, bullet.direction.y, 0.0)
            * bullet.speed
            * time.delta_secs();

        // Remove bullets that go off screen
        if transform.translation.x.abs() > 600.0 || transform.translation.y.abs() > 400.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn collision_system(
    mut enemy_query: Query<(Entity, &mut Enemy, &Transform), Without<Bullet>>,
    bullet_query: Query<(Entity, &Bullet, &Transform), Without<Enemy>>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (bullet_entity, bullet, bullet_transform) in bullet_query.iter() {
        for (enemy_entity, mut enemy, enemy_transform) in enemy_query.iter_mut() {
            let distance = bullet_transform
                .translation
                .distance(enemy_transform.translation);

            if distance < 16.0 {
                // Collision threshold
                // Damage enemy
                enemy.health -= bullet.damage;

                // Remove bullet
                commands.entity(bullet_entity).despawn();

                // Play hit sound and create visual effect
                play_console_beep(SoundType::Hit);
                create_sound_effect_visual(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    enemy_transform.translation,
                    SoundType::Hit,
                );

                // Create hit effect
                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(8.0))),
                    MeshMaterial2d(materials.add(Color::srgb(1.0, 0.8, 0.2))),
                    Transform::from_translation(
                        enemy_transform.translation + Vec3::new(0.0, 0.0, 2.0),
                    ),
                    HitEffect {
                        timer: Timer::from_seconds(0.2, TimerMode::Once),
                    },
                ));

                // Check if enemy is dead
                if enemy.health <= 0.0 {
                    // Play death sound and create visual effect
                    play_console_beep(SoundType::Death);
                    create_sound_effect_visual(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        enemy_transform.translation,
                        SoundType::Death,
                    );

                    // Create explosion effect
                    for i in 0..6 {
                        let angle = i as f32 * std::f32::consts::PI * 2.0 / 6.0;
                        let offset = Vec3::new(angle.cos() * 15.0, angle.sin() * 15.0, 2.0);
                        commands.spawn((
                            Mesh2d(meshes.add(Circle::new(3.0))),
                            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.4, 0.1))),
                            Transform::from_translation(enemy_transform.translation + offset),
                            ExplosionParticle {
                                velocity: Vec2::new(angle.cos() * 80.0, angle.sin() * 80.0),
                                timer: Timer::from_seconds(0.5, TimerMode::Once),
                            },
                        ));
                    }

                    commands.entity(enemy_entity).despawn();
                    game_state.score += 10;
                    game_state.money += 5;
                }

                break;
            }
        }
    }
}
