use crate::graphics::color::Color;
use crate::graphics::line::Line;
use crate::graphics::rectangle::Rectangle;

use super::line;

pub fn render(buffer: &mut [u8], width: u32, height: u32, rectangle: &Rectangle, color: &Color) {
    let width = width as i32;
    let height = height as i32;

    let mut x = rectangle.x.round() as i32;
    let mut y = rectangle.y.round() as i32;

    if x >= width || y >= height {
        return;
    }

    let mut w = rectangle.width.round() as i32;
    let mut h = rectangle.height.round() as i32;

    if x < 0 {
        w = w + x;
        x = 0;
    }

    if y < 0 {
        h = h + y;
        y = 0;
    }

    if w <= 0 || h <= 0 {
        return;
    }

    if x + w > width {
        w = width - x;
    }

    if y + h > height {
        h = height - y;
    }

    let length = w * h * 4;

    let mut point = 0;

    loop {
        let i = point / 4;
        let x = x + i % w;
        let y = y + i / w;

        let index = (x + y * width) * 4;

        buffer[index as usize + 0] = color.red;
        buffer[index as usize + 1] = color.green;
        buffer[index as usize + 2] = color.blue;
        buffer[index as usize + 3] = color.alpha;

        point += 4;

        if point >= length {
            break;
        }
    }
}

pub fn render_outline(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    rectangle: &Rectangle,
    color: &Color,
) {
    let x = rectangle.x;
    let y = rectangle.y;

    let w = rectangle.width;
    let h = rectangle.height;

    let width = width as u32;
    let height = height as u32;

    let left = Line::new(x, y, x, y + h);
    let right = Line::new(x + w, y, x + w, y + h);
    let top = Line::new(x, y, x + w, y);
    let bottom = Line::new(x, y + h, x + w, y + h);

    line::render(buffer, width, height, &left, color);
    line::render(buffer, width, height, &right, color);
    line::render(buffer, width, height, &top, color);
    line::render(buffer, width, height, &bottom, color);
}
