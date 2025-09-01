use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
    #[allow(dead_code)] // Used for future health bar implementation
    pub max_health: f32,
    pub speed: f32,
    pub path_progress: f32,
}

#[derive(Component)]
pub struct Tower {
    pub damage: f32,
    pub range: f32,
    pub fire_rate: f32,
    pub last_shot: f32,
}

#[derive(Component)]
pub struct Bullet {
    pub damage: f32,
    pub speed: f32,
    pub direction: Vec2,
}

#[allow(dead_code)] // Placeholder for future features
#[derive(Component)]
pub struct Health(pub f32);

#[allow(dead_code)] // Placeholder for future features
#[derive(Component)]
pub struct PathFollower {
    pub progress: f32,
    pub speed: f32,
}

#[allow(dead_code)] // Placeholder for future features
#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct HealthText;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct HitEffect {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ExplosionParticle {
    pub velocity: Vec2,
    pub timer: Timer,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_creation() {
        let enemy = Enemy {
            health: 100.0,
            max_health: 100.0,
            speed: 50.0,
            path_progress: 0.0,
        };

        assert_eq!(enemy.health, 100.0);
        assert_eq!(enemy.max_health, 100.0);
        assert_eq!(enemy.speed, 50.0);
        assert_eq!(enemy.path_progress, 0.0);
    }

    #[test]
    fn test_tower_creation() {
        let tower = Tower {
            damage: 25.0,
            range: 100.0,
            fire_rate: 1.0,
            last_shot: 0.0,
        };

        assert_eq!(tower.damage, 25.0);
        assert_eq!(tower.range, 100.0);
        assert_eq!(tower.fire_rate, 1.0);
        assert_eq!(tower.last_shot, 0.0);
    }

    #[test]
    fn test_bullet_creation() {
        let bullet = Bullet {
            damage: 25.0,
            speed: 300.0,
            direction: Vec2::new(1.0, 0.0),
        };

        assert_eq!(bullet.damage, 25.0);
        assert_eq!(bullet.speed, 300.0);
        assert_eq!(bullet.direction, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_enemy_damage() {
        let mut enemy = Enemy {
            health: 100.0,
            max_health: 100.0,
            speed: 50.0,
            path_progress: 0.0,
        };

        // Simulate taking damage
        enemy.health -= 25.0;
        assert_eq!(enemy.health, 75.0);

        // Check if enemy should be dead
        enemy.health -= 100.0;
        assert!(enemy.health <= 0.0);
    }

    #[test]
    fn test_hit_effect_creation() {
        let hit_effect = HitEffect {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        };

        assert!(!hit_effect.timer.finished());
        assert_eq!(hit_effect.timer.duration().as_secs_f32(), 0.2);
    }

    #[test]
    fn test_explosion_particle_creation() {
        let particle = ExplosionParticle {
            velocity: Vec2::new(80.0, 60.0),
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        };

        assert_eq!(particle.velocity, Vec2::new(80.0, 60.0));
        assert_eq!(particle.timer.duration().as_secs_f32(), 0.5);
        assert!(!particle.timer.finished());
    }
}
