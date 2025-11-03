#[derive(Clone, Debug)]
pub struct AgentConfig {
    pub hidden_sizes: Vec<usize>,
    pub activation: ActivationFunction,
    pub learning_rate: f32,
    pub clip_epsilon: f32,
    pub entropy_coef: f32,
    pub value_coef: f32,
    pub gamma: f32,
    pub gae_lambda: f32,
    pub batch_size: usize,
    pub epochs: usize,
    pub horizon: usize,
    pub exploration_noise: f32,
    pub min_exploration: f32,
}

#[derive(Clone, Debug)]
pub enum ActivationFunction {
    ReLU,
    Tanh,
    SiLU,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            hidden_sizes: vec![64, 32], // Réseau plus capable
            activation: ActivationFunction::Tanh,
            learning_rate: 0.0005, // Plus rapide
            clip_epsilon: 0.1, // Plus strict
            entropy_coef: 0.05, // Moins d'exploration
            value_coef: 0.5,
            gamma: 0.98,
            gae_lambda: 0.95,
            batch_size: 128, // Plus grand
            epochs: 10, // Plus d'entraînement
            horizon: 2048,
            exploration_noise: 0.5, // Beaucoup d'exploration initiale
            min_exploration: 0.05,
        }
    }
}