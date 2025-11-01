use bevy::prelude::*;
use crate::game::rocket::RocketStats;
use crate::game::GameState;

#[derive(Component)]
pub struct StatsUI;

#[derive(Component)]
pub struct GameStateUI;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
           .add_systems(Update, (update_ui, update_game_state_ui));
    }
}

fn setup_ui(mut commands: Commands) {
    // UI des statistiques (coin supérieur gauche)
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "🚀 SIMULATEUR D'ATTERRISSAGE RL\n\n",
                TextStyle {
                    font_size: 18.0,
                    color: Color::rgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            TextSection::new("Altitude: ", TextStyle { font_size: 16.0, color: Color::WHITE, ..default() }),
            TextSection::new("0.0 m\n", TextStyle { font_size: 16.0, color: Color::YELLOW, ..default() }),
            TextSection::new("Vitesse Verticale: ", TextStyle { font_size: 16.0, color: Color::WHITE, ..default() }),
            TextSection::new("0.0 m/s\n", TextStyle { font_size: 16.0, color: Color::CYAN, ..default() }),
            TextSection::new("Vitesse Horizontale: ", TextStyle { font_size: 16.0, color: Color::WHITE, ..default() }),
            TextSection::new("0.0 m/s\n", TextStyle { font_size: 16.0, color: Color::CYAN, ..default() }),
            TextSection::new("Angle: ", TextStyle { font_size: 16.0, color: Color::WHITE, ..default() }),
            TextSection::new("0.0°\n", TextStyle { font_size: 16.0, color: Color::ORANGE, ..default() }),
            TextSection::new("Carburant: ", TextStyle { font_size: 16.0, color: Color::WHITE, ..default() }),
            TextSection::new("100%\n", TextStyle { font_size: 16.0, color: Color::GREEN, ..default() }),
            TextSection::new("Distance Cible: ", TextStyle { font_size: 16.0, color: Color::WHITE, ..default() }),
            TextSection::new("0.0 m\n\n", TextStyle { font_size: 16.0, color: Color::PURPLE, ..default() }),
            TextSection::new(
                "CONDITIONS STRICTES OBLIGATOIRES:\n• Zone JAUNE: -40 à +40\n• Angle: 81° à 99° (90° ± 10%)\n• Vitesse V < 3 m/s\n• Vitesse H < 1 m/s\n\n",
                TextStyle {
                    font_size: 12.0,
                    color: Color::rgb(1.0, 0.8, 0.8),
                    ..default()
                },
            ),
            TextSection::new(
                "STATISTIQUES RL:\nAtterrissages: 0\nCrashes: 0\nSuccès consécutifs: 0\nTaille: +0%",
                TextStyle {
                    font_size: 12.0,
                    color: Color::rgb(0.8, 1.0, 0.8),
                    ..default()
                },
            ),
            TextSection::new(
                "\nTOUTE ERREUR = CRASH\nGravité réduite ✓",
                TextStyle {
                    font_size: 11.0,
                    color: Color::rgb(1.0, 0.5, 0.5),
                    ..default()
                },
            ),
        ])
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        StatsUI,
    ));

    // UI État du jeu (coin inférieur droit)
    commands.spawn((
        TextBundle::from_sections([TextSection::new(
            "🎮 EN VOL",
            TextStyle {
                font_size: 24.0,
                color: Color::GREEN,
                ..default()
            },
        )])
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
        GameStateUI,
    ));
}

fn update_ui(
    stats: Res<RocketStats>,
    mut ui_query: Query<&mut Text, With<StatsUI>>,
) {
    if let Ok(mut text) = ui_query.get_single_mut() {
        if text.sections.len() >= 17 {
            text.sections[2].value = format!("{:.1} m\n", stats.altitude);
            text.sections[4].value = format!("{:.1} m/s\n", stats.vertical_speed);
            text.sections[6].value = format!("{:.1} m/s\n", stats.horizontal_speed);
            text.sections[8].value = format!("{:.1}°\n", stats.angle.to_degrees().abs());
            text.sections[10].value = format!("{:.0}%\n", stats.fuel_percentage * 100.0);
            text.sections[12].value = format!("{:.1} m\n\n", stats.distance_to_target);
            
            // Met à jour les statistiques RL
            let size_increase = (stats.consecutive_successes as f32 * 5.0).min(30.0);
            text.sections[15].value = format!(
                "STATISTIQUES RL:\nAtterrissages: {}\nCrashes: {}\nSuccès consécutifs: {}\nTaille: +{:.0}%",
                stats.total_landings, stats.total_crashes, stats.consecutive_successes, size_increase
            );
            
            // Avertissement strict
            text.sections[16].value = format!(
                "\nTOUTE ERREUR = CRASH\nAngle requis: 81°-99°\nZone: -40 à +40",
            );
        }
    }
}

fn update_game_state_ui(
    game_state: Res<GameState>,
    mut ui_query: Query<&mut Text, With<GameStateUI>>,
) {
    if let Ok(mut text) = ui_query.get_single_mut() {
        match *game_state {
            GameState::Playing => {
                text.sections[0].value = "🎮 EN VOL".to_string();
                text.sections[0].style.color = Color::GREEN;
            }
            GameState::Crashed => {
                text.sections[0].value = "💥 CRASH".to_string();
                text.sections[0].style.color = Color::RED;
            }
            GameState::Landed => {
                text.sections[0].value = "🎯 RÉUSSI".to_string();
                text.sections[0].style.color = Color::GOLD;
            }
            GameState::Restarting => {
                text.sections[0].value = "🔄 REDÉMARRAGE".to_string();
                text.sections[0].style.color = Color::YELLOW;
            }
        }
    }
}