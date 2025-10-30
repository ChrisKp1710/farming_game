// src/sprite_test.rs
use bevy::prelude::*;

/// Plugin per testare gli sprite
pub struct SpriteTestPlugin;

impl Plugin for SpriteTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_test_sprites);
    }
}

/// Sistema per creare sprite di test
fn spawn_test_sprites(mut commands: Commands) {
    // Sprite semplice - un quadrato verde (il nostro "player" temporaneo)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.8, 0.2), // Verde per il player
            custom_size: Some(Vec2::new(32.0, 32.0)), // 32x32 pixel
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0), // Al centro del mondo
        TestPlayer, // Tag per identificarlo
    ));

    // Sprite di sfondo - quadrato marrone (terreno)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.6, 0.4, 0.2), // Marrone per il terreno
            custom_size: Some(Vec2::new(64.0, 64.0)), // Più grande
            ..default()
        },
        Transform::from_xyz(50.0, -30.0, 0.0), // Spostato a destra e in basso
        TestGround, // Tag per identificarlo
    ));

    // Altro sprite - quadrato blu (acqua)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.2, 0.4, 0.8), // Blu per l'acqua
            custom_size: Some(Vec2::new(48.0, 48.0)),
            ..default()
        },
        Transform::from_xyz(-60.0, 40.0, 0.0), // Spostato a sinistra e in alto
        TestWater, // Tag per identificarlo
    ));

    info!("🎨 Sprite di test creati!");
    info!("🟢 Quadrato verde = Player temporaneo");
    info!("🟤 Quadrato marrone = Terreno");  
    info!("🔵 Quadrato blu = Acqua");
}

/// Componenti per identificare i diversi sprite di test
#[derive(Component)]
pub struct TestPlayer;

#[derive(Component)]
pub struct TestGround;

#[derive(Component)]
pub struct TestWater;