use crate::graphics::graphic::Graphic;

use super::background;
use super::line;
use super::point;
use super::rectangle;

pub fn render(buffer: &mut [u8], width: u32, height: u32, graphics: &Vec<&Graphic>) {
    for graphic in graphics.iter() {
        match graphic {
            Graphic::Background { color } => {
                background::render(buffer, color);
            }
            Graphic::Point { point, color } => {
                point::render(buffer, width, height, point, color);
            }
            Graphic::Line { line, color } => {
                line::render(buffer, width, height, line, color);
            }
            Graphic::Rectangle { rectangle, color } => {
                if rectangle.solid {
                    rectangle::render(buffer, width, height, rectangle, color);
                } else {
                    rectangle::render_outline(buffer, width, height, rectangle, color);
                }
            }
            _ => {}
        }
    }
}
