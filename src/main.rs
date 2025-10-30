use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn camera 2D
    commands.spawn(Camera2d);
    
    println!("Bevy funziona! 🦀🎮");
}