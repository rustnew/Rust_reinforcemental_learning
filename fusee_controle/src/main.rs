pub mod game;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use game::RocketGamePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🚀 Rocket Landing Simulator".into(),
                resolution: WindowResolution::new(1400.0, 1000.0),
                resizable: true, // Permet de redimensionner
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RocketGamePlugin)
        .run();
}