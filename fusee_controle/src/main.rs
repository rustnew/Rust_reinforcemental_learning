pub mod game;
pub mod rl_agent;

use bevy::prelude::*;
use bevy::window::WindowResolution;
use game::RocketGamePlugin;
use rl_agent::RLAgentPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "🚀 Rocket Landing Simulator - RL Agent".into(),
                resolution: WindowResolution::new(1200.0, 800.0),
                resizable: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RocketGamePlugin)
        .add_plugins(RLAgentPlugin)
        .run();
}