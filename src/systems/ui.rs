use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn update_ui(
    game_state: Res<GameState>,
    mut health_text_query: Query<&mut Text, (With<HealthText>, Without<ScoreText>)>,
    mut score_text_query: Query<&mut Text, (With<ScoreText>, Without<HealthText>)>,
) {
    if let Ok(mut text) = health_text_query.get_single_mut() {
        **text = format!("Health: {}", game_state.player_health);
    }
    
    if let Ok(mut text) = score_text_query.get_single_mut() {
        **text = format!("Score: {} | Money: {}", game_state.score, game_state.money);
    }
}