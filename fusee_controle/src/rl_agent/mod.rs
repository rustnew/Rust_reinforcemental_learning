pub mod ppo;
pub mod network;
pub mod memory;
pub mod normalizer;
pub mod config;

use bevy::prelude::*;
use crate::game::rocket::RocketStats;
use crate::game::GameState;
use ppo::PPOAgent;
use config::AgentConfig;
use memory::Transition;

#[derive(Component, Default)]
pub struct RocketControls {
    pub throttle: f32,
    pub rotation: f32,
    pub controlled_by_ai: bool,
}

#[derive(Resource)]
pub struct RLTraining {
    pub agent: PPOAgent,
    pub episode_count: u32,
    pub total_steps: u64,
    pub best_score: f32,
    pub training: bool,
    pub current_episode_steps: usize,
    pub last_state: Option<Vec<f32>>,
    pub last_action: Option<Vec<f32>>,
    pub consecutive_crashes: u32,
}

pub struct RLAgentPlugin;

impl Plugin for RLAgentPlugin {
    fn build(&self, app: &mut App) {
        let config = AgentConfig::default();
        let agent = PPOAgent::new(config);
        
        app.insert_resource(RLTraining {
            agent,
            episode_count: 0,
            total_steps: 0,
            best_score: 0.0,
            training: true,
            current_episode_steps: 0,
            last_state: None,
            last_action: None,
            consecutive_crashes: 0,
        })
        .add_systems(Startup, setup_ai_controls)
        .add_systems(Update, (rl_control_system, training_log_system, handle_episode_end));
    }
}

fn setup_ai_controls(mut commands: Commands) {
    commands.spawn(RocketControls {
        throttle: 0.0,
        rotation: 0.0,
        controlled_by_ai: true,
    });
}

    
    fn rl_control_system(
    mut training: ResMut<RLTraining>,
    game_state: Res<GameState>,
    stats: Res<RocketStats>,
    mut controls_query: Query<&mut RocketControls>,
) {
    if *game_state != GameState::Playing || !training.training {
        return;
    }

    let observation = vec![
        stats.altitude / 400.0,
        (stats.vertical_speed + 200.0) / 400.0, // Normalisation améliorée
        stats.horizontal_speed / 100.0,
        stats.angle.to_degrees() / 180.0,
        stats.fuel_percentage,
        (stats.distance_to_target / 400.0).min(1.0),
    ];

    // Gérer la transition précédente
    if let (Some(last_state), Some(last_action)) = (training.last_state.take(), training.last_action.take()) {
        let reward = training.agent.compute_reward(&stats, &game_state, training.current_episode_steps);
        let done = matches!(*game_state, GameState::Landed | GameState::Crashed);
        
        let transition = Transition {
            state: last_state,
            action: last_action.clone(),
            reward,
            next_state: observation.clone(),
            done,
            log_prob: 0.0,
            value: 0.0,
        };
        
        training.agent.memory.push(transition);
        
        // LOG seulement pour comportements intéressants
        if reward.abs() > 5.0 || training.total_steps % 300 == 0 {
            println!("🤖 State - Alt: {:.1}, V: {:.1}, Angle: {:.1}°, Dist: {:.1}, Reward: {:.2}", 
                    stats.altitude, stats.vertical_speed, stats.angle.to_degrees(), 
                    stats.distance_to_target, reward);
        }
    }

    // Obtenir nouvelle action
    let action = training.agent.get_action(&observation);
    
    // Appliquer contrôles
    if let Ok(mut controls) = controls_query.get_single_mut() {
        if controls.controlled_by_ai {
            controls.throttle = (action[0] + 1.0) / 2.0;
            controls.rotation = action[1].clamp(-1.0, 1.0);
        }
    }

    training.last_state = Some(observation);
    training.last_action = Some(action.clone());
    
    training.total_steps += 1;
    training.current_episode_steps += 1;
}

fn handle_episode_end(
    mut training: ResMut<RLTraining>,
    game_state: Res<GameState>,
    stats: Res<RocketStats>,
) {
    if matches!(*game_state, GameState::Landed | GameState::Crashed) {
        let episode = training.episode_count;
        let score = stats.landing_score;
        
        // Gérer les crashes consécutifs
        if *game_state == GameState::Crashed {
            training.consecutive_crashes += 1;
        } else {
            training.consecutive_crashes = 0;
        }
        
        println!("🎯 Episode {} - Score: {:.1}, Crashes consécutifs: {}", 
                episode, score, training.consecutive_crashes);
        
        // Extraire valeurs pour éviter double emprunt
        let episode_count = training.episode_count;
        let best_score = training.best_score;
        
        // Vérifier stagnation seulement périodiquement
        if episode_count % 50 == 0 {
            training.agent.check_performance_stagnation(episode_count, best_score);
        }
        
        // Entraîner seulement si assez de données
        if training.agent.memory.size >= training.agent.config.batch_size {
            training.agent.train_from_memory();
        }
        
        training.episode_count += 1;
        
        if *game_state == GameState::Landed && score > training.best_score {
            training.best_score = score;
            println!("🏆 NEW BEST SCORE: {:.1}", training.best_score);
        }
        
        // Réinitialiser épisode
        training.current_episode_steps = 0;
        training.last_state = None;
        training.last_action = None;
    }
}

fn training_log_system(
    training: Res<RLTraining>,
    game_state: Res<GameState>,
) {
    if matches!(*game_state, GameState::Landed | GameState::Crashed) {
        if training.episode_count % 20 == 0 {
            println!("🤖 RL Agent - Episode: {}, Best Score: {:.1}, Total Steps: {}, Memory: {}/{}",
                    training.episode_count, training.best_score, training.total_steps,
                    training.agent.memory.size, training.agent.memory.capacity);
        }
    }
}