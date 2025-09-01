use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy {
    pub health: f32,
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

#[derive(Component)]
pub struct Health(pub f32);

#[derive(Component)]
pub struct PathFollower {
    pub progress: f32,
    pub speed: f32,
}

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