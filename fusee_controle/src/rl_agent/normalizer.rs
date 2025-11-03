#[derive(Clone, Debug)]
pub struct RunningNormalizer {
    pub mean: Vec<f32>,
    pub variance: Vec<f32>,
    pub count: f32,
    pub epsilon: f32,
}

impl RunningNormalizer {
    pub fn new(num_features: usize) -> Self {
        Self {
            mean: vec![0.0; num_features],
            variance: vec![1.0; num_features],
            count: 0.0,
            epsilon: 1e-8,
        }
    }

    pub fn update(&mut self, state: &[f32]) {
        self.count += 1.0;
        let count = self.count;

        // Mise à jour de la moyenne
        for i in 0..state.len() {
            let delta = state[i] - self.mean[i];
            self.mean[i] += delta / count;
        }

        // Mise à jour de la variance
        for i in 0..state.len() {
            let delta = state[i] - self.mean[i];
            self.variance[i] += delta * delta;
        }
    }

    pub fn normalize(&self, state: &[f32]) -> Vec<f32> {
        state.iter().enumerate().map(|(i, &x)| {
            if self.count < 2.0 {
                x
            } else {
                let std = (self.variance[i] / (self.count - 1.0)).sqrt().max(self.epsilon);
                (x - self.mean[i]) / std
            }
        }).collect()
    }

    pub fn reset(&mut self) {
        self.mean.iter_mut().for_each(|x| *x = 0.0);
        self.variance.iter_mut().for_each(|x| *x = 1.0);
        self.count = 0.0;
    }
}