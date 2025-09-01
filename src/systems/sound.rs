use bevy::prelude::*;

/// Simple sound event system that doesn't rely on external files
/// We'll use a minimalist approach with visual feedback instead of complex audio generation

#[derive(Component)]
pub struct SoundEvent {
    pub sound_type: SoundType,
    pub timer: Timer,
}

#[derive(Clone, Copy)]
pub enum SoundType {
    Shoot,
    Death,
    Hit,
}

/// Instead of complex audio generation, we'll create visual sound indicators
/// This is much simpler and doesn't require external dependencies
pub fn handle_sound_events(
    mut commands: Commands,
    mut sound_query: Query<(Entity, &mut SoundEvent)>,
    time: Res<Time>,
) {
    for (entity, mut sound_event) in sound_query.iter_mut() {
        sound_event.timer.tick(time.delta());
        
        if sound_event.timer.just_finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Create a simple beep effect using console output (cross-platform)
pub fn play_console_beep(sound_type: SoundType) {
    match sound_type {
        SoundType::Shoot => {
            // Simple console beep for shooting
            print!("\x07"); // ASCII bell character
        },
        SoundType::Death => {
            // Double beep for death
            print!("\x07\x07");
        },
        SoundType::Hit => {
            // Quick beep for hit
            print!("\x07");
        },
    }
}

/// Alternative: Create visual sound effects instead of audio
pub fn create_sound_effect_visual(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec3,
    sound_type: SoundType,
) {
    let (color, size) = match sound_type {
        SoundType::Shoot => (Color::srgb(1.0, 1.0, 0.8), 3.0), // Light yellow
        SoundType::Death => (Color::srgb(1.0, 0.6, 0.6), 8.0), // Light red  
        SoundType::Hit => (Color::srgb(0.8, 1.0, 0.8), 4.0),   // Light green
    };
    
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(size))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_translation(position),
        SoundEvent {
            sound_type,
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        },
    ));
}