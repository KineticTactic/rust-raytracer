use std::f64;

#[derive(Clone)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub const REAL: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    pub const POSITIVE: Interval = Interval {
        min: 0.0,
        max: f64::INFINITY,
    };

    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const INTENSITY: Interval = Interval {
        min: 0.0,
        max: 0.999,
    };

    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}
