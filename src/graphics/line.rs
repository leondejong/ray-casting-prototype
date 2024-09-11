use super::point::Point;

#[derive(Debug, Copy, Clone, Default)]
pub struct Line {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl Line {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self { x0, y0, x1, y1 }
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        let a = Point::new(self.x0, self.y0);
        let b = Point::new(self.x1, self.y1);
        let c = Point::new(x, y);
        let margin = 0.5;
        a.distance_point(c) + b.distance_point(c) - a.distance_point(b) < margin
    }
}
