use crate::components::*;
use crate::resources::*;
use bevy::prelude::*;

pub fn spawn_enemies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut wave_timer: ResMut<WaveTimer>,
) {
    wave_timer.timer.tick(time.delta());

    if wave_timer.timer.just_finished() {
        let start_pos = Vec3::new(ENEMY_PATH[0].0, ENEMY_PATH[0].1, 1.0);

        commands.spawn((
            Mesh2d(meshes.add(Circle::new(12.0))),
            MeshMaterial2d(materials.add(Color::srgb(1.0, 0.3, 0.3))),
            Transform::from_translation(start_pos),
            Enemy {
                health: 100.0,
                max_health: 100.0,
                speed: 50.0,
                path_progress: 0.0,
            },
        ));
    }
}

pub fn move_enemies(
    mut enemy_query: Query<(Entity, &mut Transform, &mut Enemy)>,
    time: Res<Time>,
    mut game_state: ResMut<GameState>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut enemy) in enemy_query.iter_mut() {
        enemy.path_progress += enemy.speed * time.delta_secs();

        let total_path_length = calculate_total_path_length();
        let progress_ratio = enemy.path_progress / total_path_length;

        if progress_ratio >= 1.0 {
            // Enemy reached the end
            game_state.player_health -= 1;
            commands.entity(entity).despawn();
            continue;
        }

        // Calculate position along path
        let new_pos = calculate_position_on_path(enemy.path_progress);
        transform.translation = new_pos;
    }
}

pub fn cleanup_dead_entities(enemy_query: Query<(Entity, &Enemy)>, mut commands: Commands) {
    for (entity, enemy) in enemy_query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn calculate_total_path_length() -> f32 {
    let mut total_length = 0.0;
    for i in 0..ENEMY_PATH.len() - 1 {
        let start = Vec2::new(ENEMY_PATH[i].0, ENEMY_PATH[i].1);
        let end = Vec2::new(ENEMY_PATH[i + 1].0, ENEMY_PATH[i + 1].1);
        total_length += start.distance(end);
    }
    total_length
}

fn calculate_position_on_path(progress: f32) -> Vec3 {
    let mut current_progress = progress;

    for i in 0..ENEMY_PATH.len() - 1 {
        let start = Vec2::new(ENEMY_PATH[i].0, ENEMY_PATH[i].1);
        let end = Vec2::new(ENEMY_PATH[i + 1].0, ENEMY_PATH[i + 1].1);
        let segment_length = start.distance(end);

        if current_progress <= segment_length {
            let t = current_progress / segment_length;
            let pos = start.lerp(end, t);
            return Vec3::new(pos.x, pos.y, 1.0);
        }

        current_progress -= segment_length;
    }

    // If we reach here, return the end position
    Vec3::new(
        ENEMY_PATH[ENEMY_PATH.len() - 1].0,
        ENEMY_PATH[ENEMY_PATH.len() - 1].1,
        1.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_total_path_length() {
        let total_length = calculate_total_path_length();

        // Path should have positive length
        assert!(total_length > 0.0);

        // Manually calculate expected length for validation
        let expected_segments = [
            // (-400, 200) to (-200, 200): horizontal 200 units
            200.0, // (-200, 200) to (-200, -100): vertical 300 units
            300.0, // (-200, -100) to (200, -100): horizontal 400 units
            400.0, // (200, -100) to (200, 100): vertical 200 units
            200.0, // (200, 100) to (400, 100): horizontal 200 units
            200.0,
        ];

        let expected_total: f32 = expected_segments.iter().sum();
        assert!((total_length - expected_total).abs() < 0.1);
    }

    #[test]
    fn test_calculate_position_on_path_start() {
        let start_pos = calculate_position_on_path(0.0);

        assert_eq!(start_pos.x, ENEMY_PATH[0].0);
        assert_eq!(start_pos.y, ENEMY_PATH[0].1);
        assert_eq!(start_pos.z, 1.0);
    }

    #[test]
    fn test_calculate_position_on_path_end() {
        let total_length = calculate_total_path_length();
        let end_pos = calculate_position_on_path(total_length);

        let last_waypoint = ENEMY_PATH[ENEMY_PATH.len() - 1];
        assert_eq!(end_pos.x, last_waypoint.0);
        assert_eq!(end_pos.y, last_waypoint.1);
        assert_eq!(end_pos.z, 1.0);
    }

    #[test]
    fn test_calculate_position_on_path_middle() {
        // Test position halfway through first segment
        let pos = calculate_position_on_path(100.0);

        // Should be halfway between (-400, 200) and (-200, 200)
        assert_eq!(pos.x, -300.0);
        assert_eq!(pos.y, 200.0);
        assert_eq!(pos.z, 1.0);
    }

    #[test]
    fn test_calculate_position_beyond_path() {
        let total_length = calculate_total_path_length();
        let beyond_pos = calculate_position_on_path(total_length + 100.0);

        // Should return the final waypoint position
        let last_waypoint = ENEMY_PATH[ENEMY_PATH.len() - 1];
        assert_eq!(beyond_pos.x, last_waypoint.0);
        assert_eq!(beyond_pos.y, last_waypoint.1);
        assert_eq!(beyond_pos.z, 1.0);
    }

    #[test]
    fn test_enemy_path_segments() {
        // Test that each segment in the path makes sense
        for i in 0..ENEMY_PATH.len() - 1 {
            let start = Vec2::new(ENEMY_PATH[i].0, ENEMY_PATH[i].1);
            let end = Vec2::new(ENEMY_PATH[i + 1].0, ENEMY_PATH[i + 1].1);
            let distance = start.distance(end);

            // Each segment should have positive length
            assert!(
                distance > 0.0,
                "Path segment {i} should have positive length"
            );

            // No segment should be unreasonably long
            assert!(
                distance < 1000.0,
                "Path segment {i} is too long: {distance}"
            );
        }
    }

    #[test]
    fn test_enemy_constants() {
        // Test that enemy constants are reasonable
        let enemy = Enemy {
            health: 100.0,
            max_health: 100.0,
            speed: 50.0,
            path_progress: 0.0,
        };

        assert!(enemy.health > 0.0);
        assert!(enemy.max_health > 0.0);
        assert!(enemy.speed > 0.0);
        assert!(enemy.path_progress >= 0.0);

        // Health should not exceed max health initially
        assert!(enemy.health <= enemy.max_health);
    }

    #[test]
    fn test_path_progression() {
        let total_length = calculate_total_path_length();

        // Test various progression points
        let test_points = [
            0.0,
            total_length * 0.25,
            total_length * 0.5,
            total_length * 0.75,
            total_length,
        ];

        for &progress in &test_points {
            let pos = calculate_position_on_path(progress);

            // All positions should have z = 1.0
            assert_eq!(pos.z, 1.0);

            // Positions should be within reasonable bounds
            assert!(pos.x >= -500.0 && pos.x <= 500.0);
            assert!(pos.y >= -200.0 && pos.y <= 300.0);
        }
    }
}
