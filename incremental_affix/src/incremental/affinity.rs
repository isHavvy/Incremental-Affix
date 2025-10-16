use std::time::Duration;

use rand::random;

#[derive(Debug, Clone, Copy)]
pub struct Affinity {
    pub chance: f64,
    pub multiplier: f64,
    pub time: Duration,
}

impl Affinity {
    pub fn new() -> Self {
        Affinity {
            chance: 0.0,
            multiplier: 0.0,
            time: Duration::ZERO,
        }
    }

    pub fn check(&self) -> bool {
        random::<f64>() < self.chance
    }
}

impl Default for Affinity {
    fn default() -> Self {
        Self::new()
    }
}