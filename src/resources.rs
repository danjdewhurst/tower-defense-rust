use bevy::prelude::*;

#[derive(Resource)]
pub struct GameState {
    pub player_health: i32,
    pub score: i32,
    pub money: i32,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            player_health: 20,
            score: 0,
            money: 100,
        }
    }
}

#[derive(Resource)]
pub struct WaveTimer {
    pub timer: Timer,
}

impl Default for WaveTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(2.0, TimerMode::Repeating),
        }
    }
}

pub const ENEMY_PATH: [(f32, f32); 6] = [
    (-400.0, 200.0),
    (-200.0, 200.0),
    (-200.0, -100.0),
    (200.0, -100.0),
    (200.0, 100.0),
    (400.0, 100.0),
];