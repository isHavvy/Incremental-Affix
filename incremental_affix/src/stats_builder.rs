pub struct StatsBuilder {
    base: f64,
    offset: f64,
    multiplier: f64,
}

impl Default for StatsBuilder {
    fn default() -> Self {
        Self {
            base: 0.0,
            offset: 0.0,
            multiplier: 1.0,
        }
    }
}

impl StatsBuilder {
    pub fn set_base(&mut self, base: f64) {
        self.base = f64::max(base, self.base);
    }

    pub fn add_offset(&mut self, offset: f64) {
        self.offset += offset;
    }

    /// Adds a multiplier.
    /// 
    /// Multipliers are summed, not multiplied.
    /// Just like in Path of Exile.
    /// 
    /// ```rust
    /// let mut stats_builder = StatsBuilder::default();
    /// stats_builder.set_base(1.0);
    /// stats_builder.add_multiplier(0.2);
    /// stats_builder.add_multiplier(0.2);
    /// let stat = stats_builder.calculate();
    /// assert_eq!(stat, 1.4); // Not 1.44
    /// ```
    pub fn add_multiplier(&mut self, multiplier: f64) {
        self.multiplier += multiplier;
    }

    pub fn calculate(&self) -> f64 {
        if self.base == 0.0 {
            0.0
        } else {
            (self.base + self.offset) * self.multiplier
        }
    }
}