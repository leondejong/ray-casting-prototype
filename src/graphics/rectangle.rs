#[derive(Debug, Copy, Clone, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub solid: bool,
}

impl Rectangle {
    pub fn new(x: f32, y: f32, width: f32, height: f32, solid: bool) -> Self {
        Self {
            x,
            y,
            width,
            height,
            solid,
        }
    }
    pub fn from_array(r: &[i32; 4], solid: bool) -> Rectangle {
        Rectangle::new(r[0] as f32, r[1] as f32, r[2] as f32, r[3] as f32, solid)
    }
    pub fn from_tuple(r: &(i32, i32, u32, u32), solid: bool) -> Rectangle {
        Rectangle::new(r.0 as f32, r.1 as f32, r.2 as f32, r.3 as f32, solid)
    }
    pub fn contains(&self, x: f32, y: f32) -> bool {
        x >= self.x && y >= self.y && x <= self.x + self.width && y <= self.y + self.height
    }
}
