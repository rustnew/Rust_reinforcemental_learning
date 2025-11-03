use rand::seq::IteratorRandom;

#[derive(Clone, Debug)]
pub struct Transition {
    pub state: Vec<f32>,
    pub action: Vec<f32>,
    pub reward: f32,
    pub next_state: Vec<f32>,
    pub done: bool,
    pub log_prob: f32,
    pub value: f32,
}

pub struct ReplayBuffer {
    pub buffer: Vec<Transition>,
    pub capacity: usize,
    pub position: usize,
    pub size: usize,
}

impl ReplayBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            capacity,
            position: 0,
            size: 0,
        }
    }

    pub fn push(&mut self, transition: Transition) {
        if self.size < self.capacity {
            self.buffer.push(transition);
            self.size += 1;
        } else {
            self.buffer[self.position] = transition;
            self.position = (self.position + 1) % self.capacity;
        }
    }

    pub fn sample(&self, batch_size: usize) -> Vec<&Transition> {
        let mut rng = rand::rng();
        self.buffer.iter().choose_multiple(&mut rng, batch_size)
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.position = 0;
        self.size = 0;
    }

    pub fn is_full(&self) -> bool {
        self.size >= self.capacity
    }
}