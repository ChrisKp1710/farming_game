// src/camera.rs
use bevy::prelude::*;

/// Plugin per gestire la camera 2D del gioco
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

/// Componente per identificare la camera principale del gioco
#[derive(Component)]
pub struct GameCamera;

/// Sistema per creare e configurare la camera 2D
fn setup_camera(mut commands: Commands) {
    // Spawn della camera 2D con configurazione ottimale per pixel art
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0), // Z alto per vedere tutto
        OrthographicProjection {
            scale: 1.0, // Zoom iniziale (1.0 = dimensione normale)
            ..OrthographicProjection::default_2d()
        },
        GameCamera, // Tag per identificare questa camera
    ));

    info!("Camera 2D inizializzata correttamente! 📷");
}

/// Configurazione consigliata per pixel art
#[allow(dead_code)]
pub struct CameraConfig;

#[allow(dead_code)]
impl CameraConfig {
    /// Scala consigliata per pixel art nitida
    pub const PIXEL_PERFECT_SCALE: f32 = 2.0;
    
    /// Velocità di movimento camera (per debug o cinematiche)
    pub const MOVEMENT_SPEED: f32 = 500.0;
    
    /// Limiti di zoom min/max
    pub const MIN_ZOOM: f32 = 0.5;
    pub const MAX_ZOOM: f32 = 4.0;
}

// Funzioni utility per gestire la camera (opzionali, per il futuro)
#[allow(dead_code)]
impl GameCamera {
    /// Ottieni la transform della camera principale
    pub fn get_transform(
        camera_query: &Query<&Transform, With<GameCamera>>,
    ) -> Option<Transform> {
        camera_query.get_single().ok().copied()
    }
    
    /// Converte coordinate schermo in coordinate mondo
    pub fn screen_to_world(
        screen_pos: Vec2,
        camera_transform: &GlobalTransform,
        camera: &Camera,
        _window: &Window,
    ) -> Option<Vec2> {
        camera
            .viewport_to_world_2d(camera_transform, screen_pos)
            .ok()
    }
}