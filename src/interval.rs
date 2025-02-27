use core::f64;

#[derive(Debug, Copy, Clone, Default)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

#[macro_export]
macro_rules! interval {
    ($min:expr, $max: expr) => {
        Interval {
            min: $min,
            max: $max,
        }
    };
}

#[allow(dead_code)]
impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        interval![min, max]
    }

    pub fn empty() -> Interval {
        interval![f64::INFINITY, f64::NEG_INFINITY]
    }

    pub fn universe() -> Interval {
        interval![f64::NEG_INFINITY, f64::INFINITY]
    }

    pub fn size(self) -> f64 {
        self.max - self.min 
    }

    pub fn contains(self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(self, x: f64) -> f64 {
        if x < self.min { return self.min };
        if x > self.max { return self.max };
        x
    }
}
