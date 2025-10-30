// Importe les traits et types de base de Bevy
use bevy::prelude::*;

/// Composant représentant les propriétés principales de la fusée
#[derive(Component)]
pub struct Rocket {
    pub fuel: f32,           // Quantité actuelle de carburant
    pub max_fuel: f32,       // Quantité maximale de carburant
    pub throttle: f32,       // Niveau de poussée (0.0 à 1.0)
    pub engine_power: f32,   // Puissance maximale du moteur
    pub rotation_speed: f32, // Vitesse de rotation de la fusée
}

/// Marqueur pour le corps principal de la fusée
#[derive(Component)]
pub struct RocketMainBody;

/// Marqueur pour la flamme du moteur
#[derive(Component)]
pub struct RocketFlame;

/// Composant pour les jambes d'atterrissage
#[derive(Component)]
pub struct LandingLegs {
    pub deployed: bool,  // Si les jambes sont déployées
    pub contact: bool,   // Si les jambes touchent le sol
}

/// Ressource stockant les statistiques actuelles de la fusée
#[derive(Resource)]
pub struct RocketStats {
    pub altitude: f32,           // Hauteur actuelle
    pub vertical_speed: f32,     // Vitesse verticale
    pub horizontal_speed: f32,   // Vitesse horizontale
    pub angle: f32,              // Angle d'inclinaison
    pub fuel_percentage: f32,    // Pourcentage de carburant restant
}

/// Plugin pour la gestion de la fusée
pub struct RocketPlugin;

// Implémente le trait Plugin pour le système de fusée
impl Plugin for RocketPlugin {
    fn build(&self, app: &mut App) {
        app
            // Initialise la ressource des statistiques de la fusée
            .insert_resource(RocketStats {
                altitude: 0.0,
                vertical_speed: 0.0,
                horizontal_speed: 0.0,
                angle: 0.0,
                fuel_percentage: 1.0,
            })
            // Ajoute le système de création de la fusée au démarrage
            .add_systems(Startup, spawn_rocket)
            // Ajoute les systèmes de mise à jour à chaque frame
            .add_systems(Update, (update_rocket_stats, update_flame_visibility));
    }
}

/// Crée la fusée et ses composants visuels
fn spawn_rocket(
    mut commands: Commands,                          // Pour créer de nouvelles entités
    mut meshes: ResMut<Assets<Mesh>>,               // Gestionnaire des meshes
    mut materials: ResMut<Assets<ColorMaterial>>,   // Gestionnaire des matériaux
) {
    // Crée le corps principal de la fusée
    commands.spawn((
        // Bundle pour un mesh 2D avec matériau
        MaterialMesh2dBundle {
            // Crée un mesh rectangulaire de 20x60 pixels
            mesh: meshes.add(Rectangle::new(20.0, 60.0)).into(),
            // Matériau blanc pour la fusée
            material: materials.add(Color::srgb(1.0, 1.0, 1.0)),
            // Position initiale au centre de l'écran
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            // Valeurs par défaut pour les autres champs
            ..default()
        },
        // Marqueur pour identifier le corps principal
        RocketMainBody,
        // Composant Rocket avec ses propriétés initiales
        Rocket {
            fuel: 100.0,        // Carburant initial
            max_fuel: 100.0,    // Capacité maximale
            throttle: 0.0,      // Poussée initiale nulle
            engine_power: 500.0, // Puissance du moteur
            rotation_speed: 2.0, // Vitesse de rotation
        },
    ));

    // Crée la flamme du moteur
    commands.spawn((
        MaterialMesh2dBundle {
            // Mesh plus petit pour la flamme
            mesh: meshes.add(Rectangle::new(15.0, 30.0)).into(),
            // Matériau orange pour la flamme
            material: materials.add(Color::srgb(1.0, 0.5, 0.0)),
            // Position sous la fusée
            transform: Transform::from_xyz(0.0, -45.0, 0.0),
            ..default()
        },
        // Marqueur pour la flamme
        RocketFlame,
    ));

    // Crée les jambes d'atterrissage gauche et droite
    spawn_landing_leg(&mut commands, &mut meshes, &mut materials, -15.0);
    spawn_landing_leg(&mut commands, &mut meshes, &mut materials, 15.0);
}

/// Crée une jambe d'atterrissage à la position spécifiée
fn spawn_landing_leg(
    commands: &mut Commands,                     // Pour créer l'entité
    meshes: &mut ResMut<Assets<Mesh>>,          // Gestionnaire des meshes
    materials: &mut ResMut<Assets<ColorMaterial>>, // Gestionnaire des matériaux
    x_offset: f32,                              // Décalage horizontal
) {
    commands.spawn((
        MaterialMesh2dBundle {
            // Mesh étroit et long pour la jambe
            mesh: meshes.add(Rectangle::new(5.0, 20.0)).into(),
            // Matériau gris pour la jambe
            material: materials.add(Color::srgb(0.5, 0.5, 0.5)),
            // Position sous la fusée avec décalage horizontal
            transform: Transform::from_xyz(x_offset, -30.0, 0.0),
            ..default()
        },
        // Composant LandingLegs avec état initial
        LandingLegs {
            deployed: false,  // Non déployée au départ
            contact: false,   // Pas de contact avec le sol
        },
    ));
}

/// Met à jour les statistiques de la fusée à chaque frame
fn update_rocket_stats(
    rocket_query: Query<(&Transform, &Rocket), With<RocketMainBody>>, // Requête pour la fusée
    mut stats: ResMut<RocketStats>, // Ressource mutable des statistiques
) {
    // Récupère la fusée (doit être une seule entité)
    if let Ok((transform, rocket)) = rocket_query.single() {
        // Met à jour l'altitude (position Y)
        stats.altitude = transform.translation.y;
        // Met à jour l'angle (rotation Z convertie en radians)
        stats.angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        // Calcule le pourcentage de carburant restant
        stats.fuel_percentage = rocket.fuel / rocket.max_fuel;
    }
}

/// Gère la visibilité de la flamme en fonction de la poussée
fn update_flame_visibility(
    rocket_query: Query<&Rocket, With<RocketMainBody>>, // Requête pour le composant Rocket
    mut flame_query: Query<&mut Visibility, With<RocketFlame>>, // Requête pour la visibilité de la flamme
) {
    // Récupère à la fois la fusée et la flamme
    if let (Ok(rocket), Ok(mut visibility)) = (rocket_query.single(), flame_query.single_mut()) {
        // Affiche la flamme seulement si la poussée est positive
        *visibility = if rocket.throttle > 0.0 {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}