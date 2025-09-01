use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

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

pub fn cleanup_dead_entities(
    enemy_query: Query<(Entity, &Enemy)>,
    mut commands: Commands,
) {
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
    Vec3::new(ENEMY_PATH[ENEMY_PATH.len() - 1].0, ENEMY_PATH[ENEMY_PATH.len() - 1].1, 1.0)
}