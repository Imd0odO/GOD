use crate::monte_carlo::linear_bounds::LinearBounds;
use crate::monte_carlo::point::Point;

#[derive(Copy, Clone)]
pub struct Bounds {
    x: LinearBounds,
    y: LinearBounds,
}

impl Bounds {
    pub fn new(x: LinearBounds, y: LinearBounds) -> Bounds {
        return Bounds {x, y};
    }

    pub fn get_area(&self) -> f64 {
        return self.x.get_length() * self.y.get_length();
    }

    pub fn get_random_point(&self) -> Point {
        return Point::new(self.x.get_random_value(), self.y.get_random_value());
    }

    pub fn get_x(&self) -> LinearBounds {
        return self.x;
    }
}
