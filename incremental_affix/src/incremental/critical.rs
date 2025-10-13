#![expect(unused)]

use rand::random;

pub struct Critical {
    pub chance: f64,
    pub multiplier: f64,
    pub length: f64,
}

impl Critical {
    pub fn new() -> Self {
        Critical {
            chance: 0.05,
            multiplier: 1.10,
            length: 1.0,
        }
    }

    pub fn check(&self) -> bool {
        random::<f64>() < self.chance
    }
}