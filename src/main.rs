use bevy::prelude::*;

mod components;
mod systems;
mod resources;

use systems::*;
use resources::*;

const WINDOW_WIDTH: f32 = 1024.0;
const WINDOW_HEIGHT: f32 = 768.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Tower Defense".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .init_resource::<WaveTimer>()
        .add_systems(Startup, setup)
        .add_systems(Update, (
            spawn_enemies,
            move_enemies,
            tower_shooting,
            bullet_movement,
            collision_system,
            cleanup_dead_entities,
            handle_input,
            update_ui,
            update_effects,
            handle_sound_events,
        ))
        .run();
}