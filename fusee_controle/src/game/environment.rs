// Importe les traits et types de base de Bevy
use bevy::prelude::*;

/// Marqueur pour le sol
#[derive(Component)]
pub struct Ground;

/// Composant pour la plateforme d'atterrissage
#[derive(Component)]
pub struct LandingPad {
    pub position: Vec2, // Position centrale de la plateforme
    pub width: f32,     // Largeur de la plateforme
}

/// Plugin pour la gestion de l'environnement
pub struct EnvironmentPlugin;

// Implémente le trait Plugin pour le système d'environnement
impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        // Ajoute le système de création de l'environnement au démarrage
        app.add_systems(Startup, spawn_environment);
    }
}

/// Crée l'environnement de jeu (sol et plateforme d'atterrissage)
fn spawn_environment(
    mut commands: Commands,                          // Pour créer de nouvelles entités
    mut meshes: ResMut<Assets<Mesh>>,               // Gestionnaire des meshes
    mut materials: ResMut<Assets<ColorMaterial>>,   // Gestionnaire des matériaux
) {
    // Crée le sol
    commands.spawn((
        MaterialMesh2dBundle {
            // Crée un mesh très large et mince pour le sol
            mesh: meshes.add(Rectangle::new(1000.0, 20.0)).into(),
            // Matériau vert foncé pour le sol
            material: materials.add(Color::srgb(0.0, 0.4, 0.0)),
            // Positionne le sol en bas de l'écran
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            ..default()
        },
        // Marqueur pour identifier le sol
        Ground,
    ));

    // Crée la plateforme d'atterrissage
    commands.spawn((
        MaterialMesh2dBundle {
            // Plateforme de 80x10 pixels
            mesh: meshes.add(Rectangle::new(80.0, 10.0)).into(),
            // Matériau gris pour la plateforme
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            // Positionne la plateforme juste au-dessus du sol
            transform: Transform::from_xyz(0.0, -290.0, 1.0),
            ..default()
        },
        // Composant LandingPad avec ses propriétés
        LandingPad {
            position: Vec2::new(0.0, -290.0), // Position centrale
            width: 80.0,                       // Largeur
        },
    ));

    // Crée un repère visuel au centre de la plateforme
    commands.spawn((
        MaterialMesh2dBundle {
            // Ligne fine au centre de la plateforme
            mesh: meshes.add(Rectangle::new(60.0, 2.0)).into(),
            // Matériau blanc pour la visibilité
            material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
            // Même position que la plateforme mais calque supérieur
            transform: Transform::from_xyz(0.0, -290.0, 2.0),
            ..default()
        },
        // Pas de composant spécial - juste visuel
    ));
}