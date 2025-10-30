// src/player.rs
use bevy::prelude::*;
use crate::sprite_test::TestPlayer;

/// Plugin per gestire il movimento e controlli del player
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            player_movement,
            player_input_debug,
        ));
    }
}

/// Componente per definire la velocità del player
#[allow(dead_code)]
#[derive(Component)]
pub struct PlayerSpeed {
    pub speed: f32,
}

impl Default for PlayerSpeed {
    fn default() -> Self {
        Self {
            speed: 200.0, // pixel al secondo
        }
    }
}

/// Sistema per gestire il movimento del player con WASD
fn player_movement(
    mut player_query: Query<&mut Transform, With<TestPlayer>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let speed = 200.0; // pixel al secondo
        let movement_delta = speed * time.delta_secs();
        
        // Input WASD
        if input.pressed(KeyCode::KeyW) || input.pressed(KeyCode::ArrowUp) {
            transform.translation.y += movement_delta;
        }
        if input.pressed(KeyCode::KeyS) || input.pressed(KeyCode::ArrowDown) {
            transform.translation.y -= movement_delta;
        }
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            transform.translation.x -= movement_delta;
        }
        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            transform.translation.x += movement_delta;
        }
    }
}

/// Sistema per debug dell'input (opzionale - mostra quando premi tasti)
fn player_input_debug(
    input: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<TestPlayer>>,
) {
    // Mostra posizione quando premi Spazio
    if input.just_pressed(KeyCode::Space) {
        if let Ok(transform) = player_query.get_single() {
            info!(
                "🎯 Player position: x={:.1}, y={:.1}", 
                transform.translation.x, 
                transform.translation.y
            );
        }
    }
    
    // Messaggio di aiuto quando premi H
    if input.just_pressed(KeyCode::KeyH) {
        info!("🎮 CONTROLLI:");
        info!("   WASD o Frecce = Movimento");
        info!("   Spazio = Mostra posizione");
        info!("   H = Mostra questo aiuto");
    }
}

/// Configurazione controlli
#[allow(dead_code)]
pub struct PlayerConfig;

#[allow(dead_code)]
impl PlayerConfig {
    pub const DEFAULT_SPEED: f32 = 200.0;
    pub const FAST_SPEED: f32 = 400.0;
    pub const SLOW_SPEED: f32 = 100.0;
}