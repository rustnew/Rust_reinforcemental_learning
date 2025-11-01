use bevy::prelude::*;
use crate::game::rocket::{Rocket, RocketMainBody};
use crate::game::GameState;

#[derive(Component)]
pub struct PhysicsBody {
    pub velocity: Vec2,
    pub angular_velocity: f32,
}

// CONSTANTES PHYSIQUES AJUSTÉES - GRAVITÉ RÉDUITE
const GRAVITY: f32 = -9.81 * 15.0; // RÉDUIT de 25 à 15 (moins rapide)
const AIR_RESISTANCE: f32 = 0.008;  // RÉSISTANCE RÉDUITE

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_physics, apply_rocket_thrust));
    }
}

fn apply_physics(
    mut query: Query<(&mut Transform, &mut PhysicsBody)>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Playing {
        return;
    }

    for (mut transform, mut body) in query.iter_mut() {
        // Gravité plus douce
        body.velocity.y += GRAVITY * time.delta_seconds();
        
        // Résistance de l'air réduite
        body.velocity *= 1.0 - (AIR_RESISTANCE * time.delta_seconds());
        
        // Limites de vitesse réalistes
        body.velocity = body.velocity.clamp_length_max(200.0);
        
        // Mise à jour position
        transform.translation.x += body.velocity.x * time.delta_seconds();
        transform.translation.y += body.velocity.y * time.delta_seconds();
        
        // Mise à jour rotation
        transform.rotate_z(body.angular_velocity * time.delta_seconds());
        
        // Limite la rotation angulaire
        body.angular_velocity = body.angular_velocity.clamp(-3.0, 3.0);
    }
}

fn apply_rocket_thrust(
    mut query: Query<(&mut PhysicsBody, &Rocket, &Transform), With<RocketMainBody>>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Playing {
        return;
    }

    if let Ok((mut physics, rocket, transform)) = query.get_single_mut() {
        if rocket.throttle > 0.0 && rocket.fuel > 0.0 && !rocket.has_crashed && !rocket.has_landed {
            let thrust_power = rocket.throttle * rocket.engine_power;
            let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
            
            // Poussée réaliste basée sur l'orientation
            physics.velocity.x += thrust_power * angle.sin() * time.delta_seconds();
            physics.velocity.y += thrust_power * angle.cos() * time.delta_seconds();
        }
    }
}