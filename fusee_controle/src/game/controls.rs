use bevy::prelude::*;
use crate::game::rocket::{Rocket, RocketMainBody};
use crate::game::physics::PhysicsBody;
use crate::game::GameState;
use crate::rl_agent::RocketControls;

pub struct RocketControlsPlugin;

impl Plugin for RocketControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            keyboard_controls,
            apply_controls,
            consume_fuel,
        ));
    }
}

// Dans keyboard_controls, s'assurer que les contrôles AI ne sont pas écrasés
fn keyboard_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut controls_query: Query<&mut RocketControls>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Playing {
        return;
    }

    if let Ok(mut controls) = controls_query.get_single_mut() {
        let mut manual_control = false;

        // Contrôles manuels
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Space) {
            controls.throttle = 1.0;
            controls.controlled_by_ai = false;
            manual_control = true;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            controls.throttle = 0.0;
            controls.controlled_by_ai = false;
            manual_control = true;
        }

        if keyboard_input.pressed(KeyCode::Left) {
            controls.rotation = 1.0;
            controls.controlled_by_ai = false;
            manual_control = true;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            controls.rotation = -1.0;
            controls.controlled_by_ai = false;
            manual_control = true;
        }

        // Retour au mode AI si pas de contrôle manuel
        if !manual_control && !controls.controlled_by_ai {
            controls.controlled_by_ai = true;
        }
    }
}

fn apply_controls(
    mut rocket_query: Query<(&mut Rocket, &mut PhysicsBody), With<RocketMainBody>>,
    controls_query: Query<&RocketControls>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Playing {
        return;
    }

    if let (Ok((mut rocket, mut physics)), Ok(controls)) = (rocket_query.get_single_mut(), controls_query.get_single()) {
        if rocket.has_crashed || rocket.has_landed {
            return;
        }

        // Appliquer la poussée
        rocket.throttle = controls.throttle;
        
        // Appliquer la rotation
        physics.angular_velocity = controls.rotation * rocket.rotation_speed;
    }
}

fn consume_fuel(
    mut rocket_query: Query<&mut Rocket>,
    time: Res<Time>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Playing {
        return;
    }

    if let Ok(mut rocket) = rocket_query.get_single_mut() {
        if rocket.throttle > 0.0 && rocket.fuel > 0.0 && !rocket.has_crashed && !rocket.has_landed {
            let fuel_consumption = rocket.throttle * 15.0 * time.delta_seconds();
            rocket.fuel -= fuel_consumption;
            rocket.fuel = rocket.fuel.max(0.0);
            
            if rocket.fuel <= 0.0 {
                rocket.throttle = 0.0;
                println!("⛽ PLUS DE CARBURANT!");
            }
        }
    }
}