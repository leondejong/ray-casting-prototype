#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x.round() == self.x.round() && y.round() == self.y.round()
    }
    pub fn distance(&self, x: f32, y: f32) -> f32 {
        ((x - self.x).powf(2.0) + (y - self.y).powf(2.0)).sqrt()
    }
    pub fn distance_point(&self, point: Point) -> f32 {
        self.distance(point.x, point.y)
    }
}
