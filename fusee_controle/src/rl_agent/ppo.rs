use super::*;
use rand::Rng;
use crate::rl_agent::network::NeuralNetwork;
use crate::rl_agent::memory::ReplayBuffer;
use crate::rl_agent::normalizer::RunningNormalizer;

pub struct PPOAgent {
    pub policy_net: NeuralNetwork,
    pub value_net: NeuralNetwork,
    pub config: AgentConfig,
    pub memory: ReplayBuffer,
    pub normalizer: RunningNormalizer,
    pub exploration_noise: f32,
    pub training_iterations: u32,
}

impl PPOAgent {
    pub fn new(config: AgentConfig) -> Self {
        let obs_size = 6;
        let action_size = 2;
        
        let mut policy_sizes = vec![obs_size];
        policy_sizes.extend(&config.hidden_sizes);
        policy_sizes.push(action_size);
        
        let mut value_sizes = vec![obs_size];
        value_sizes.extend(&config.hidden_sizes);
        value_sizes.push(1);
        
        Self {
            policy_net: NeuralNetwork::new(&policy_sizes, config.activation.clone()),
            value_net: NeuralNetwork::new(&value_sizes, config.activation.clone()),
            config: config.clone(),
            memory: ReplayBuffer::new(5000), // Réduit la capacité
            normalizer: RunningNormalizer::new(obs_size),
            exploration_noise: config.exploration_noise,
            training_iterations: 0,
        }
    }

    pub fn get_action(&mut self, state: &[f32]) -> Vec<f32> {
        self.normalizer.update(state);
        let normalized_state = self.normalizer.normalize(state);
        
        let mut action = self.policy_net.forward(&normalized_state);
        
        // Exploration adaptative
        let effective_noise = if self.training_iterations < 1000 {
            self.exploration_noise * 3.0 // Beaucoup d'exploration au début
        } else {
            self.exploration_noise
        };
        
        let mut rng = rand::rng();
        for i in 0..action.len() {
            action[i] = (action[i] + rng.random_range(-1.0..1.0) * effective_noise)
                .clamp(-1.0, 1.0);
        }
        
        action
    }

    pub fn compute_reward(
    &self,
    stats: &RocketStats,
    game_state: &GameState,
    step: usize
) -> f32 {
    let mut reward = 0.0;

    match game_state {
        GameState::Landed => {
            // RÉCOMPENSE MASSIVE SEULEMENT pour atterrissage parfait
            reward += 1000.0;
            reward += stats.landing_score * 10.0;
            println!("🎉 LANDING SUCCESS! Score: {:.1}", stats.landing_score);
        }
        GameState::Crashed => {
            // PÉNALITÉ FORTE pour crash
            reward -= 100.0;
            
            // Pénalités spécifiques pour guider l'apprentissage
            let angle_deviation = (stats.angle.to_degrees().abs() - 90.0).abs();
            reward -= angle_deviation * 2.0; // Forte pénalité pour mauvais angle
            
            reward -= stats.vertical_speed.abs() * 3.0; // Forte pénalité pour vitesse verticale
            reward -= stats.horizontal_speed.abs() * 2.0; // Pénalité vitesse horizontale
            
            if stats.distance_to_target > 50.0 {
                reward -= 20.0; // Pénalité pour être loin de la zone
            }
        }
        GameState::Playing => {
            // RÉCOMPENSES TRÈS EXIGEANTES - seulement pour bon comportement
            
            // 1. PÉNALITÉ DE BASE pour encourager l'action rapide
            reward -= 0.1;
            
            // 2. RÉCOMPENSE CRITIQUE : ANGLE VERTICAL (81-99°)
            let angle_deviation = (stats.angle.to_degrees().abs() - 90.0).abs();
            if angle_deviation <= 9.0 {
                reward += 10.0; // FORTE récompense pour bon angle
            } else if angle_deviation <= 30.0 {
                reward += 2.0; // Petite récompense pour angle acceptable
            } else {
                reward -= 5.0; // Pénalité pour mauvais angle
            }
            
            // 3. RÉCOMPENSE : VITESSE VERTICALE CONTRÔLÉE
            if stats.vertical_speed.abs() < 10.0 {
                reward += 5.0; // Récompense pour vitesse très lente
            } else if stats.vertical_speed.abs() < 30.0 {
                reward += 1.0; // Petite récompense
            } else if stats.vertical_speed > 50.0 {
                reward -= 3.0; // Pénalité pour monter trop vite
            }
            
            // 4. RÉCOMPENSE : POSITION (seulement si angle et vitesse sont bons)
            if angle_deviation <= 30.0 && stats.vertical_speed.abs() < 50.0 {
                if stats.distance_to_target < 40.0 {
                    reward += 5.0; // Forte récompense pour être dans la zone
                } else if stats.distance_to_target < 100.0 {
                    reward += 1.0; // Petite récompense pour être proche
                }
            }
            
            // 5. RÉCOMPENSE : DESCENTE PROGRESSIVE
            if stats.altitude < 100.0 && stats.vertical_speed < 0.0 {
                reward += 2.0; // Récompense pour descendre en basse altitude
            }
            
            // 6. CARBURANT - récompense modérée
            reward += stats.fuel_percentage * 0.5;
        }
        _ => {}
    }

    reward
}

    pub fn train_from_memory(&mut self) {
    if self.memory.size < self.config.batch_size {
        return;
    }

    let batch = self.memory.sample(self.config.batch_size);
    
    let total_reward: f32 = batch.iter().map(|t| t.reward).sum();
    let avg_reward = total_reward / batch.len() as f32;
    
    let positive_rewards: usize = batch.iter().filter(|t| t.reward > 0.0).count();
    let success_rate = positive_rewards as f32 / batch.len() as f32;
    
    // ANALYSE DÉTAILLÉE des performances
    let high_rewards: usize = batch.iter().filter(|t| t.reward > 5.0).count();
    let high_reward_rate = high_rewards as f32 / batch.len() as f32;
    
    println!("🧠 Training - Avg Reward: {:.2}, Success Rate: {:.1}%, High Rewards: {:.1}%", 
             avg_reward, success_rate * 100.0, high_reward_rate * 100.0);
    
    // STRATÉGIE BASÉE SUR LES RÉCOMPENSES ÉLEVÉES
    if high_reward_rate > 0.3 {
        // Excellentes performances - mutation très légère
        self.policy_net = self.policy_net.copy_with_mutation(0.05, 0.005);
        println!("✅ Excellent performance - minimal mutation");
    } else if high_reward_rate > 0.1 {
        // Bonnes performances - mutation légère
        self.policy_net = self.policy_net.copy_with_mutation(0.1, 0.01);
        println!("👍 Good performance - light mutation");
    } else if avg_reward > 0.0 {
        // Performances moyennes - mutation modérée
        self.policy_net = self.policy_net.copy_with_mutation(0.2, 0.05);
        println!("⚠️ Moderate performance - medium mutation");
    } else {
        // Mauvaises performances - mutation agressive
        self.policy_net = self.policy_net.copy_with_mutation(0.4, 0.1);
        self.exploration_noise = (self.exploration_noise * 1.3).min(0.8);
        println!("🔁 Poor performance - aggressive mutation + exploration");
    }

    self.training_iterations += 1;
    
    // Réduction d'exploration plus lente
    self.exploration_noise = (self.exploration_noise * 0.999)
        .max(self.config.min_exploration);

    // Vider mémoire seulement si pleine et performances stables
    if self.memory.size >= self.memory.capacity && avg_reward > 0.0 {
        println!("🗑️ Clearing memory - good performance detected");
        self.memory.clear();
    }
}

    pub fn check_performance_stagnation(&mut self, episode_count: u32, best_score: f32) -> bool {
        // Réinitialiser seulement après 500 épisodes sans succès
        if episode_count > 500 && best_score == 0.0 {
            println!("🔄 PERFORMANCE STAGNATION - Resetting agent after {} episodes", episode_count);
            
            let obs_size = 6;
            let action_size = 2;
            let mut policy_sizes = vec![obs_size];
            policy_sizes.extend(&self.config.hidden_sizes);
            policy_sizes.push(action_size);
            
            let mut value_sizes = vec![obs_size];
            value_sizes.extend(&self.config.hidden_sizes);
            value_sizes.push(1);
            
            self.policy_net = NeuralNetwork::new(&policy_sizes, self.config.activation.clone());
            self.value_net = NeuralNetwork::new(&value_sizes, self.config.activation.clone());
            self.exploration_noise = self.config.exploration_noise * 2.0; // Plus d'exploration après reset
            self.normalizer.reset();
            self.training_iterations = 0;
            
            return true;
        }
        false
    }

    fn compute_advantages(&self, rewards: &[f32], dones: &[bool]) -> Vec<f32> {
        let mut advantages = vec![0.0; rewards.len()];
        let mut running_return = 0.0;

        for i in (0..rewards.len()).rev() {
            if dones[i] {
                running_return = rewards[i];
            } else {
                running_return = rewards[i] + self.config.gamma * running_return;
            }
            advantages[i] = running_return;
        }

        advantages
    }
}