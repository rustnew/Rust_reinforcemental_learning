pub mod rocket;
pub mod physics;
pub mod controls;
pub mod environment;
pub mod ui;

use bevy::prelude::*;
use rocket::RocketPlugin;
use controls::RocketControlsPlugin;
use physics::PhysicsPlugin;
use ui::UIPlugin;
use environment::EnvironmentPlugin;

pub struct RocketGamePlugin;

impl Plugin for RocketGamePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                PhysicsPlugin,
                RocketPlugin,
                RocketControlsPlugin,
                UIPlugin,
                EnvironmentPlugin,
            ))
            .add_systems(Startup, (setup_camera, startup_message))
            .add_systems(Update, game_state_system)
            .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.1)))
            .insert_resource(GameState::Playing);
    }
}

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub enum GameState {
    Playing,
    Crashed,
    Landed,
    Restarting,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        ..default()
    });
}


fn startup_message() {
    println!("
🎮 ROCKET LANDING SIMULATOR - CONDITIONS STRICTES
=================================================
CONDITIONS D'ATTERRISSAGE OBLIGATOIRES:
• Zone d'atterrissage: -40 à +40 (rectangle JAUNE)
• Angle parfait: 81° à 99° seulement (90° ± 10%)
• Vitesse verticale < 3 m/s
• Vitesse horizontale < 1 m/s
=================================================
RÈGLE STRICTE:
• TOUTE condition non respectée = CRASH immédiat
• Pas de tolérance d'erreur
• Feedback détaillé des erreurs
=================================================
    ");
}

fn game_state_system(
    mut game_state: ResMut<GameState>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    match *game_state {
        GameState::Crashed | GameState::Landed => {
            if keyboard_input.just_pressed(KeyCode::R) {
                println!("🔄 REDÉMARRAGE MANUEL!");
                *game_state = GameState::Restarting;
            }
        }
        _ => {}
    }
}