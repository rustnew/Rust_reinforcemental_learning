use bevy::prelude::*;
use crate::game::rocket::{Rocket, RocketMainBody};
use crate::game::physics::PhysicsBody;
use crate::game::GameState;

pub struct RocketControlsPlugin;

impl Plugin for RocketControlsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            keyboard_controls,
            apply_rotation,
            consume_fuel,
        ));
    }
}

fn keyboard_controls(
    keyboard_input: Res<Input<KeyCode>>,
    mut rocket_query: Query<&mut Rocket, With<RocketMainBody>>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Playing {
        return;
    }

    if let Ok(mut rocket) = rocket_query.get_single_mut() {
        if rocket.has_crashed || rocket.has_landed {
            return;
        }

        // Contrôles progressifs pour plus de précision
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::Space) {
            rocket.throttle = (rocket.throttle + 1.5 * 0.016).min(1.0);
        } else {
            rocket.throttle = (rocket.throttle - 1.0 * 0.016).max(0.0);
        }
    }
}

fn apply_rotation(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut PhysicsBody, &Rocket), With<RocketMainBody>>,
    game_state: Res<GameState>,
) {
    if *game_state != GameState::Playing {
        return;
    }

    if let Ok((mut physics, rocket)) = query.get_single_mut() {
        if rocket.has_crashed || rocket.has_landed {
            physics.angular_velocity = 0.0;
            return;
        }

        let rotation_power = rocket.rotation_speed;
        
        if keyboard_input.pressed(KeyCode::Left) {
            physics.angular_velocity = rotation_power;
        } else if keyboard_input.pressed(KeyCode::Right) {
            physics.angular_velocity = -rotation_power;
        } else {
            physics.angular_velocity *= 0.85; // Amortissement réaliste
        }
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