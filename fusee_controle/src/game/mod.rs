// Déclare tous les sous-modules du jeu
pub mod rocket;      // Gestion de la fusée et ses composants
pub mod physics;     // Système physique
pub mod controls;    // Contrôles manuels
pub mod environment; // Environnement de jeu (sol, plateforme)
pub mod ui;          // Interface utilisateur

// Importe les traits et types de base de Bevy
use bevy::prelude::*;
// Importe les plugins de chaque module
use crate::game::controls::RocketControlsPlugin;
use crate::game::physics::PhysicsPlugin;
use crate::game::rocket::RocketPlugin;
use crate::game::ui::UIPlugin;
use crate::game::environment::EnvironmentPlugin;

/// Plugin principal qui regroupe tous les plugins du jeu
pub struct RocketGamePlugin;

// Implémente le trait Plugin pour notre plugin principal
impl Plugin for RocketGamePlugin {
    /// Configure l'application en ajoutant tous les systèmes et ressources
    fn build(&self, app: &mut App) {
        app
            // Ajoute tous les plugins enfants
            .add_plugins((
                PhysicsPlugin,          // Gestion de la physique
                RocketPlugin,           // Gestion de la fusée
                RocketControlsPlugin,   // Contrôles manuels
                UIPlugin,               // Interface utilisateur
                EnvironmentPlugin,      // Environnement
            ))
            // Ajoute le système de configuration de la caméra au démarrage
            .add_systems(Startup, setup_camera)
            // Définit la couleur de fond de l'écran (bleu foncé)
            .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.2)));
    }
}

/// Configure la caméra 2D au démarrage du jeu
fn setup_camera(mut commands: Commands) {
    // Crée une entité avec un bundle de caméra 2D
    commands.spawn(Camera2dBundle {
        // Positionne la caméra à (0,0,10) pour voir la scène
        transform: Transform::from_xyz(0.0, 0.0, 10.0),
        // Utilise les valeurs par défaut pour les autres composants
        ..default()
    });
}