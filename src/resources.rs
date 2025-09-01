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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_state_default() {
        let game_state = GameState::default();

        assert_eq!(game_state.player_health, 20);
        assert_eq!(game_state.score, 0);
        assert_eq!(game_state.money, 100);
    }

    #[test]
    fn test_game_state_operations() {
        let mut game_state = GameState::default();

        // Test taking damage
        game_state.player_health -= 1;
        assert_eq!(game_state.player_health, 19);

        // Test scoring
        game_state.score += 10;
        assert_eq!(game_state.score, 10);

        // Test spending money
        game_state.money -= 20;
        assert_eq!(game_state.money, 80);

        // Test earning money
        game_state.money += 5;
        assert_eq!(game_state.money, 85);
    }

    #[test]
    fn test_wave_timer_default() {
        let wave_timer = WaveTimer::default();

        assert_eq!(wave_timer.timer.duration().as_secs_f32(), 2.0);
        assert!(!wave_timer.timer.finished());
    }

    #[test]
    fn test_enemy_path_validity() {
        // Test that we have the expected number of waypoints
        assert_eq!(ENEMY_PATH.len(), 6);

        // Test that path starts and ends at reasonable positions
        let start = ENEMY_PATH[0];
        let end = ENEMY_PATH[ENEMY_PATH.len() - 1];

        assert_eq!(start, (-400.0, 200.0));
        assert_eq!(end, (400.0, 100.0));

        // Test that all path points are reasonable (within expected bounds)
        for (x, y) in ENEMY_PATH {
            assert!(
                (-500.0..=500.0).contains(&x),
                "X coordinate {x} out of bounds"
            );
            assert!(
                (-200.0..=300.0).contains(&y),
                "Y coordinate {y} out of bounds"
            );
        }
    }

    #[test]
    fn test_path_segments() {
        use bevy::math::Vec2;

        // Calculate total path length for testing
        let mut total_length = 0.0;
        for i in 0..ENEMY_PATH.len() - 1 {
            let start = Vec2::new(ENEMY_PATH[i].0, ENEMY_PATH[i].1);
            let end = Vec2::new(ENEMY_PATH[i + 1].0, ENEMY_PATH[i + 1].1);
            total_length += start.distance(end);
        }

        // Path should be reasonably long for a tower defense game
        assert!(total_length > 0.0);
        assert!(total_length < 2000.0); // Not too long
    }

    #[test]
    fn test_game_balance() {
        let game_state = GameState::default();

        // Test initial game balance
        assert!(
            game_state.money >= 20,
            "Player should start with enough money for at least one tower"
        );
        assert!(
            game_state.player_health > 0,
            "Player should start with positive health"
        );

        // Test that initial money allows for tower placement
        let tower_cost = 20;
        let max_towers = game_state.money / tower_cost;
        assert!(
            max_towers >= 1,
            "Should be able to place at least one tower initially"
        );
    }
}
