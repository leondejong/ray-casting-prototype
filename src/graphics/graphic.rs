use super::color::Color;
use super::line::Line;
use super::point::Point;
use super::rectangle::Rectangle;

#[derive(Debug, Clone, Default)]
pub enum Graphic {
    #[default]
    None,
    Background {
        color: Color,
    },
    Point {
        point: Point,
        color: Color,
    },
    Line {
        line: Line,
        color: Color,
    },
    Rectangle {
        rectangle: Rectangle,
        color: Color,
    },
}

impl Graphic {
    pub fn none() -> Self {
        Graphic::None
    }
    pub fn background(color: Color) -> Self {
        Graphic::Background { color }
    }
    pub fn point(point: Point, color: Color) -> Self {
        Graphic::Point { point, color }
    }
    pub fn line(line: Line, color: Color) -> Self {
        Graphic::Line { line, color }
    }
    pub fn rectangle(rectangle: Rectangle, color: Color) -> Self {
        Graphic::Rectangle { rectangle, color }
    }
}

impl Graphic {
    pub fn set_x_y(&mut self, x: f32, y: f32) {
        match self {
            Graphic::Point { point, .. } => {
                point.x = x;
                point.y = y;
            }
            Graphic::Line { line, .. } => {
                let dx = line.x1 - line.x0;
                let dy = line.y1 - line.y0;
                line.x0 = x;
                line.y0 = y;
                line.x1 = dx + x;
                line.y1 = dy + y;
            }
            Graphic::Rectangle { rectangle, .. } => {
                rectangle.x = x;
                rectangle.y = y;
            }
            _ => {}
        }
    }
}
