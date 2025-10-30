// src/main.rs
use bevy::prelude::*;
mod camera;

use camera::CameraPlugin;

fn main() {
    App::new()
        // Plugin di base di Bevy
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🌾 Farming Game".into(),
                resolution: (1280.0, 720.0).into(),
                present_mode: bevy::window::PresentMode::AutoVsync,
                // Opzioni per pixel art nitida
                canvas: None,
                fit_canvas_to_parent: false,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        
        // Il nostro plugin camera
        .add_plugins(CameraPlugin)
        
        // Sistema di startup per messaggi di debug
        .add_systems(Startup, setup_debug)
        
        // Avvia l'app
        .run();
}

/// Sistema per setup iniziale e messaggi di debug
fn setup_debug() {
    println!("Bevy funziona! 🦀🎮");
    info!("🌾 Farming Game inizializzato!");
    info!("📝 Camera 2D attiva");
    info!("🎯 Pronto per aggiungere il player!");
}