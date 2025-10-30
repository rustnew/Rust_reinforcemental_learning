// Importe les traits et types de base de Bevy
use bevy::prelude::*;
// Importe la ressource des statistiques de la fusée
use crate::game::rocket::RocketStats;

/// Marqueur pour l'interface utilisateur des statistiques
#[derive(Component)]
pub struct StatsUI;

/// Plugin pour la gestion de l'interface utilisateur
pub struct UIPlugin;

// Implémente le trait Plugin pour le système d'interface
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            // Ajoute le système de création de l'UI au démarrage
            .add_systems(Startup, setup_ui)
            // Ajoute le système de mise à jour de l'UI à chaque frame
            .add_systems(Update, update_ui);
    }
}

/// Crée l'interface utilisateur avec les statistiques et instructions
fn setup_ui(mut commands: Commands) {
    // Crée l'entité texte pour l'interface
    commands.spawn((
        // Bundle de texte avec plusieurs sections formatées
        TextBundle::from_sections([
            // Section 0: Titre du jeu
            TextSection::new(
                "Rocket Landing Simulator\n\n", // Texte du titre
                TextStyle {
                    font_size: 24.0,   // Taille de police grande
                    color: Color::srgb(1.0, 1.0, 1.0), // Couleur blanche
                    ..default()        // Autres valeurs par défaut
                },
            ),
            // Section 1: Libellé "Altitude: "
            TextSection::new(
                "Altitude: ",
                TextStyle {
                    font_size: 18.0,   // Taille moyenne
                    color: Color::srgb(1.0, 1.0, 1.0), 
                    ..default()
                },
            ),
            // Section 2: Valeur de l'altitude (sera mise à jour)
            TextSection::new(
                "0.0\n", // Valeur initiale
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 1.0, 0.0), // Valeur en jaune pour distinction
                    ..default()
                },
            ),
            // Section 3: Libellé "Vitesse Verticale: "
            TextSection::new(
                "Vitesse Verticale: ",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            // Section 4: Valeur de la vitesse verticale
            TextSection::new(
                "0.0\n",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 1.0, 0.0),
                    ..default()
                },
            ),
            // Section 5: Libellé "Angle: "
            TextSection::new(
                "Angle: ",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            // Section 6: Valeur de l'angle
            TextSection::new(
                "0.0\n",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 1.0, 0.0),
                    ..default()
                },
            ),
            // Section 7: Libellé "Carburant: "
            TextSection::new(
                "Carburant: ",
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(1.0, 1.0, 1.0),
                    ..default()
                },
            ),
            // Section 8: Valeur du carburant (pourcentage)
            TextSection::new(
                "100%\n\n", // Valeur initiale
                TextStyle {
                    font_size: 18.0,
                    color: Color::srgb(0.0, 1.0, 0.0), // Vert pour le carburant
                    ..default()
                },
            ),
            // Section 9: Instructions de contrôle
            TextSection::new(
                "Contrôles:\nFlèche Haut/Espace: Poussée\nFlèches Gauche/Droite: Rotation",
                TextStyle {
                    font_size: 16.0,   // Plus petit pour les instructions
                    color: Color::srgb(0.0, 1.0, 1.0), // Cyan pour visibilité
                    ..default()
                },
            ),
        ])
        // Style de positionnement de l'UI
        .with_style(Style {
            position_type: PositionType::Absolute, // Position absolue dans la fenêtre
            top: Val::Px(10.0),    // 10 pixels du haut
            left: Val::Px(10.0),   // 10 pixels de la gauche
            ..default()            // Autres valeurs par défaut
        }),
        // Marqueur pour identifier cette UI
        StatsUI,
    ));
}

/// Met à jour les valeurs de l'interface utilisateur avec les statistiques actuelles
fn update_ui(
    stats: Res<RocketStats>,              // Ressource des statistiques (en lecture)
    mut ui_query: Query<&mut Text, With<StatsUI>>, // Requête pour le texte de l'UI
) {
    // Récupère le composant Text de l'UI
    if let Ok(mut text) = ui_query.single_mut() {
        // Met à jour chaque valeur avec formatage
        
        // Section 2: Altitude avec 1 décimale
        text.sections[2].value = format!("{:.1}\n", stats.altitude);
        
        // Section 4: Vitesse verticale avec 1 décimale
        text.sections[4].value = format!("{:.1}\n", stats.vertical_speed);
        
        // Section 6: Angle avec 2 décimales (précision importante)
        text.sections[6].value = format!("{:.2}\n", stats.angle);
        
        // Section 8: Pourcentage de carburant sans décimale
        text.sections[8].value = format!("{:.0}%\n\n", stats.fuel_percentage * 100.0);
    }
}