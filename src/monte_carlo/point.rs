#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        return Point {x, y};
    }

    pub fn get_x(&self) -> f64 {
        return self.x;
    }

    pub fn get_y(&self) -> f64 {
        return self.y;
    }
}