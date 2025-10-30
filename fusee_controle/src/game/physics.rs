// Importe les traits et types de base de Bevy
use bevy::prelude::*;
// Importe les composants du module rocket
use crate::game::rocket::{Rocket, RocketMainBody};

// Constantes physiques
const GRAVITY: f32 = -9.81 * 50.0; // Gravité terrestre adaptée à l'échelle du jeu
const AIR_RESISTANCE: f32 = 0.1;   // Coefficient de résistance de l'air

/// Composant pour les corps physiques avec vitesse
#[derive(Component)]
pub struct PhysicsBody {
    pub velocity: Vec2,        // Vitesse linéaire (x, y)
    pub angular_velocity: f32, // Vitesse angulaire (rotation)
}

/// Plugin pour la gestion de la physique
pub struct PhysicsPlugin;

// Implémente le trait Plugin pour le système physique
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        // Ajoute les systèmes de physique à exécuter à chaque frame
        app.add_systems(Update, (apply_physics, apply_rocket_thrust));
    }
}

/// Applique la physique à tous les corps physiques
fn apply_physics(
    mut query: Query<(&mut Transform, &mut PhysicsBody)>, // Requête pour les entités avec physique
    time: Res<Time>, // Ressource donnant le temps écoulé depuis la dernière frame
) {
    // Parcourt toutes les entités avec Transform et PhysicsBody
    for (mut transform, mut body) in query.iter_mut() {
        // Applique la gravité à la vitesse verticale
        body.velocity.y += GRAVITY * time.delta_secs();
        
        // Applique la résistance de l'air (freinage)
        body.velocity *= 1.0 - AIR_RESISTANCE * time.delta_secs();
        
        // Met à jour la position en fonction de la vitesse
        transform.translation += Vec3::new(
            body.velocity.x * time.delta_secs(), // Déplacement horizontal
            body.velocity.y * time.delta_secs(), // Déplacement vertical
            0.0, // Pas de déplacement en profondeur
        );
        
        // Met à jour la rotation en fonction de la vitesse angulaire
        transform.rotate_z(body.angular_velocity * time.delta_secs());
    }
}

/// Applique la poussée de la fusée à son corps physique
fn apply_rocket_thrust(
    mut query: Query<(&mut PhysicsBody, &Rocket, &Transform), With<RocketMainBody>>, // Requête pour la fusée
    time: Res<Time>, // Ressource temps
) {
    // Récupère la fusée (doit être une seule entité)
    if let Ok((mut physics, rocket, transform)) = query.single_mut() {
        // Vérifie si le moteur est allumé et qu'il reste du carburant
        if rocket.throttle > 0.0 && rocket.fuel > 0.0 {
            // Calcule la puissance de poussée actuelle
            let thrust_power = rocket.throttle * rocket.engine_power;
            // Récupère l'angle actuel de la fusée
            let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
            
            // Calcule les composantes de la poussée en fonction de l'angle
            // Poussée horizontale (sinus de l'angle)
            physics.velocity.x += thrust_power * angle.sin() * time.delta_secs();
            // Poussée verticale (cosinus de l'angle)
            physics.velocity.y += thrust_power * angle.cos() * time.delta_secs();
        }
    }
}