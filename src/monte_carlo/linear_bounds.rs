use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};

#[derive(Copy, Clone)]
pub struct LinearBounds {
    lower: f64,
    higher: f64,
}

impl LinearBounds {
    pub fn new(lower: f64, higher: f64) -> LinearBounds {
        return LinearBounds {lower, higher};
    }

    pub fn get_length(&self) -> f64 {
        return (self.higher - self.lower).abs();
    }

    pub fn get_random_value(&self) -> f64 {
        let mut rand: ThreadRng = thread_rng();
        return self.lower + self.get_length() * rand.gen::<f64>();
    }

    pub fn get_lower(&self) -> f64 {
        return self.lower;
    }

    pub fn get_higher(&self) -> f64 {
        return self.higher;
    }
}