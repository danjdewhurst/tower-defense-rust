use bevy::prelude::*;
use crate::components::*;
use crate::resources::*;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2d);

    // Draw path
    for i in 0..ENEMY_PATH.len() - 1 {
        let start = Vec3::new(ENEMY_PATH[i].0, ENEMY_PATH[i].1, 0.0);
        let end = Vec3::new(ENEMY_PATH[i + 1].0, ENEMY_PATH[i + 1].1, 0.0);
        
        // Simple line representation using small rectangles
        let direction = (end - start).normalize();
        let distance = start.distance(end);
        let segments = (distance / 10.0) as usize;
        
        for j in 0..segments {
            let pos = start + direction * (j as f32 * 10.0);
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(8.0, 8.0))),
                MeshMaterial2d(materials.add(Color::srgb(0.7, 0.7, 0.3))),
                Transform::from_translation(pos),
            ));
        }
    }

    // UI
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|parent| {
            // Health display
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        top: Val::Px(10.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Health: 20"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        HealthText,
                    ));
                });

            // Score display
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        left: Val::Px(10.0),
                        top: Val::Px(40.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Score: 0"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                        ScoreText,
                    ));
                });

            // Instructions
            parent
                .spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        right: Val::Px(10.0),
                        bottom: Val::Px(10.0),
                        ..default()
                    },
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Click to place towers"),
                        TextFont {
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ));
                });
        });
}