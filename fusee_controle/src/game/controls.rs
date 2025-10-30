// Importe les traits et types de base de Bevy
use bevy::prelude::*;
// Importe les composants du module rocket
use crate::game::rocket::{Rocket, RocketMainBody};
// Importe le PhysicsBody du module physics
use crate::game::physics::PhysicsBody;

/// Plugin pour la gestion des contrôles manuels
pub struct RocketControlsPlugin;

// Implémente le trait Plugin pour le système de contrôles
impl Plugin for RocketControlsPlugin {
    fn build(&self, app: &mut App) {
        // Ajoute les systèmes de contrôle à exécuter à chaque frame
        app.add_systems(Update, (
            keyboard_controls,  // Gestion du clavier
            apply_rotation,     // Application de la rotation
            consume_fuel        // Consommation de carburant
        ));
    }
}

/// Gère les entrées clavier pour contrôler la fusée
fn keyboard_controls(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Ressource des entrées clavier (nouvelle API)
    mut rocket_query: Query<&mut Rocket, With<RocketMainBody>>, // Requête pour la fusée
) {
    // Récupère la fusée (doit être une seule entité)
    if let Ok(mut rocket) = rocket_query.single_mut() {
        // Contrôle de la poussée avec flèche haut ou espace
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::Space) {
            // Augmente progressivement la poussée (limitée à 1.0)
            // 0.5 * 0.016 ≈ 8% par seconde à 60 FPS
            rocket.throttle = (rocket.throttle + 0.5 * 0.016).min(1.0);
        } else {
            // Réduit progressivement la poussée (limitée à 0.0)
            rocket.throttle = (rocket.throttle - 0.3 * 0.016).max(0.0);
        }

        // Les rotations sont gérées dans apply_rotation
        // Ici on garde la structure pour éventuelles extensions futures
    }
}

/// Applique la rotation de la fusée basée sur les entrées clavier
fn apply_rotation(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Ressource des entrées clavier (nouvelle API)
    mut query: Query<(&mut PhysicsBody, &Rocket), With<RocketMainBody>>, // Requête pour la physique de la fusée
    time: Res<Time>, // Ressource temps
) {
    // Récupère la fusée (doit être une seule entité)
    if let Ok((mut physics, rocket)) = query.single_mut() {
        // Récupère la puissance de rotation de la fusée
        let rotation_power = rocket.rotation_speed;
        
        // Applique la rotation en fonction des touches pressées
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            // Rotation vers la gauche (positive en Bevy)
            physics.angular_velocity = rotation_power;
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            // Rotation vers la droite (négative en Bevy)
            physics.angular_velocity = -rotation_power;
        } else {
            // Aucune touche de rotation - amortissement progressif
            physics.angular_velocity *= 0.9; // Réduit de 10% par frame
        }
    }
}

/// Gère la consommation de carburant de la fusée
fn consume_fuel(
    mut rocket_query: Query<&mut Rocket>, // Requête pour le composant Rocket
    time: Res<Time>, // Ressource temps
) {
    // Récupère la fusée (doit être une seule entité)
    if let Ok(mut rocket) = rocket_query.single_mut() {
        // Consomme du carburant seulement si le moteur est allumé et qu'il en reste
        if rocket.throttle > 0.0 && rocket.fuel > 0.0 {
            // Consommation proportionnelle à la poussée
            // 10.0 * time.delta_seconds() ≈ 10 unités par seconde à pleine puissance
            rocket.fuel -= rocket.throttle * 10.0 * time.delta_secs();
            // Garantit que le carburant ne devient pas négatif
            rocket.fuel = rocket.fuel.max(0.0);
        }
    }
}