use std::time::Duration;

use rand::random;

#[derive(Debug)]
pub struct Affinity {
    pub chance: f64,
    pub multiplier: f64,
    pub time: Duration,
}

impl Affinity {
    pub fn new() -> Self {
        Affinity {
            chance: 0.50,
            multiplier: 2.0,
            time: Duration::from_secs(1),
        }
    }

    pub fn check(&self) -> bool {
        random::<f64>() < self.chance
    }
}