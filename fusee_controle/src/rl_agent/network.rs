use rand::Rng;
use crate::rl_agent::config::ActivationFunction;

#[derive(Clone, Debug)]
pub struct NeuralNetwork {
    pub weights: Vec<Vec<Vec<f32>>>, // layer -> input -> output
    pub biases: Vec<Vec<f32>>,       // layer -> neuron
    pub activation: ActivationFunction,
}

impl NeuralNetwork {
    pub fn new(layer_sizes: &[usize], activation: ActivationFunction) -> Self {
        let mut rng = rand::rng();
        let mut weights = Vec::new();
        let mut biases = Vec::new();

        for i in 0..layer_sizes.len() - 1 {
            let input_size = layer_sizes[i];
            let output_size = layer_sizes[i + 1];
            
            // Initialisation He normalisée
            let scale = (2.0 / input_size as f32).sqrt();
            
            let layer_weights: Vec<Vec<f32>> = (0..input_size)
                .map(|_| (0..output_size)
                    .map(|_| rng.random_range(-1.0..1.0) * scale)
                    .collect())
                .collect();
                
            let layer_biases: Vec<f32> = (0..output_size)
                .map(|_| rng.random_range(-0.1..0.1))
                .collect();

            weights.push(layer_weights);
            biases.push(layer_biases);
        }

        Self {
            weights,
            biases,
            activation,
        }
    }

    pub fn forward(&self, input: &[f32]) -> Vec<f32> {
        let mut activation = input.to_vec();

        for layer in 0..self.weights.len() {
            let mut new_activation = Vec::with_capacity(self.biases[layer].len());
            
            for neuron in 0..self.biases[layer].len() {
                let mut sum = self.biases[layer][neuron];
                
                for (input_idx, &input_val) in activation.iter().enumerate() {
                    sum += input_val * self.weights[layer][input_idx][neuron];
                }
                
                new_activation.push(self.activate(sum));
            }
            
            activation = new_activation;
        }

        activation
    }

    fn activate(&self, x: f32) -> f32 {
        match self.activation {
            ActivationFunction::ReLU => x.max(0.0),
            ActivationFunction::Tanh => x.tanh(),
            ActivationFunction::SiLU => x / (1.0 + (-x).exp()), // Swish/SiLU
        }
    }

    pub fn copy_with_mutation(&self, mutation_rate: f32, mutation_strength: f32) -> Self {
        let mut rng = rand::rng();
        let mut new_weights = self.weights.clone();
        let mut new_biases = self.biases.clone();

        for layer in 0..new_weights.len() {
            for input_idx in 0..new_weights[layer].len() {
                for output_idx in 0..new_weights[layer][input_idx].len() {
                    if rng.random::<f32>() < mutation_rate {
                        new_weights[layer][input_idx][output_idx] += 
                            rng.random_range(-1.0..1.0) * mutation_strength;
                        // Assurer que les poids restent dans des limites raisonnables
                        new_weights[layer][input_idx][output_idx] = new_weights[layer][input_idx][output_idx]
                            .clamp(-2.0, 2.0);
                    }
                }
            }

            for bias_idx in 0..new_biases[layer].len() {
                if rng.random::<f32>() < mutation_rate {
                    new_biases[layer][bias_idx] += 
                        rng.random_range(-1.0..1.0) * mutation_strength;
                    // Assurer que les biais restent dans des limites raisonnables
                    new_biases[layer][bias_idx] = new_biases[layer][bias_idx].clamp(-1.0, 1.0);
                }
            }
        }

        Self {
            weights: new_weights,
            biases: new_biases,
            activation: self.activation.clone(),
        }
    }
}