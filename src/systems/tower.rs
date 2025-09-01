use super::sound::{create_sound_effect_visual, play_console_beep, SoundType};
use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn tower_shooting(
    mut tower_query: Query<(&Transform, &mut Tower)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Tower>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
) {
    for (tower_transform, mut tower) in tower_query.iter_mut() {
        tower.last_shot += time.delta_secs();

        if tower.last_shot < 1.0 / tower.fire_rate {
            continue;
        }

        // Find closest enemy in range
        let mut closest_enemy: Option<Vec3> = None;
        let mut closest_distance = f32::MAX;

        for enemy_transform in enemy_query.iter() {
            let distance = tower_transform
                .translation
                .distance(enemy_transform.translation);
            if distance <= tower.range && distance < closest_distance {
                closest_distance = distance;
                closest_enemy = Some(enemy_transform.translation);
            }
        }

        if let Some(enemy_pos) = closest_enemy {
            tower.last_shot = 0.0;

            // Create bullet
            let direction = (enemy_pos - tower_transform.translation).normalize_or_zero();

            // Play shooting sound (console beep + visual effect)
            play_console_beep(SoundType::Shoot);
            create_sound_effect_visual(
                &mut commands,
                &mut meshes,
                &mut materials,
                tower_transform.translation,
                SoundType::Shoot,
            );

            commands.spawn((
                Mesh2d(meshes.add(Circle::new(4.0))),
                MeshMaterial2d(materials.add(Color::srgb(1.0, 1.0, 0.3))),
                Transform::from_translation(tower_transform.translation + Vec3::Z),
                Bullet {
                    damage: tower.damage,
                    speed: 300.0,
                    direction: direction.truncate(),
                },
            ));
        }
    }
}

#[allow(clippy::too_many_arguments)] // System functions often need many parameters
pub fn handle_input(
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<GameState>,
    tower_query: Query<&Transform, With<Tower>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) && game_state.money >= 20 {
        let window = windows.single();
        let (camera, camera_transform) = camera_query.single();

        if let Some(cursor_pos) = window.cursor_position() {
            let world_pos = camera
                .viewport_to_world_2d(camera_transform, cursor_pos)
                .unwrap();

            // Check if position is valid (not too close to other towers)
            let mut can_place = true;
            for tower_transform in tower_query.iter() {
                if world_pos.distance(tower_transform.translation.truncate()) < 60.0 {
                    can_place = false;
                    break;
                }
            }

            if can_place {
                game_state.money -= 20;

                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(24.0, 24.0))),
                    MeshMaterial2d(materials.add(Color::srgb(0.3, 0.7, 1.0))),
                    Transform::from_translation(world_pos.extend(1.0)),
                    Tower {
                        damage: 25.0,
                        range: 100.0,
                        fire_rate: 1.0,
                        last_shot: 0.0,
                    },
                ));
            }
        }
    }
}
