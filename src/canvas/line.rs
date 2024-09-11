use crate::graphics::color::Color;
use crate::graphics::line::Line;

use super::point;

pub fn render(buffer: &mut [u8], width: u32, height: u32, line: &Line, color: &Color) {
    let width = width as usize;
    let height = height as usize;

    let x0 = line.x0.round() as i32;
    let y0 = line.y0.round() as i32;

    let x1 = line.x1.round() as i32;
    let y1 = line.y1.round() as i32;

    if x0 == x1 && (x0 < 0 || x0 >= width as i32) {
        return;
    }

    if y0 == y1 && (y0 < 0 || y0 >= height as i32) {
        return;
    }

    let x0 = x0.clamp(0, width as i32);
    let y0 = y0.clamp(0, height as i32);

    let x1 = x1.clamp(0, width as i32);
    let y1 = y1.clamp(0, height as i32);

    if (x1 - x0).abs() > (y1 - y0).abs() {
        if x0 < x1 {
            low(buffer, width, height, color, x0, y0, x1, y1);
        } else {
            low(buffer, width, height, color, x1, y1, x0, y0);
        }
    } else {
        if y0 < y1 {
            high(buffer, width, height, color, x0, y0, x1, y1);
        } else {
            high(buffer, width, height, color, x1, y1, x0, y0);
        }
    }
}

fn low(
    buffer: &mut [u8],
    width: usize,
    height: usize,
    color: &Color,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
) {
    let dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut yi = 1;
    if dy < 0 {
        yi = -1;
        dy = -dy;
    }
    let mut d = (dy * 2) - dx;
    let mut y = y0;
    for x in x0..x1 {
        point::plot(buffer, width, height, color, x as usize, y as usize);
        if d > 0 {
            y = y + yi;
            d = d + ((dy - dx) * 2)
        } else {
            d = d + dy * 2
        }
    }
}

fn high(
    buffer: &mut [u8],
    width: usize,
    height: usize,
    color: &Color,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
) {
    let mut dx = x1 - x0;
    let dy = y1 - y0;
    let mut xi = 1;
    if dx < 0 {
        xi = -1;
        dx = -dx;
    }
    let mut d = (dx * 2) - dy;
    let mut x = x0;
    for y in y0..y1 {
        point::plot(buffer, width, height, color, x as usize, y as usize);
        if d > 0 {
            x = x + xi;
            d = d + ((dx - dy) * 2)
        } else {
            d = d + dx * 2;
        }
    }
}

pub fn horizontal(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    length: u32,
    color: &Color,
) {
    let width = width as i32;
    let height = height as i32;

    let mut length = length as i32;

    if x >= width || y >= height || (x + length) < 0 {
        return;
    }

    let mut x = x;

    if x < 0 {
        length = length + x;
        x = 0;
    }

    if x + length > width {
        length = width - x;
    }

    let mut i = 0;
    let l = length * 4;

    loop {
        let x = x + (i / 4) % length;

        let index = (x + y * width) * 4;

        buffer[index as usize + 0] = color.red;
        buffer[index as usize + 1] = color.green;
        buffer[index as usize + 2] = color.blue;
        buffer[index as usize + 3] = color.alpha;

        i += 4;

        if i >= l {
            break;
        }
    }
}

pub fn vertical(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    x: i32,
    y: i32,
    length: u32,
    color: &Color,
) {
    let width = width as i32;
    let height = height as i32;

    let mut length = length as i32;

    if x >= width || y >= height || (y + length) < 0 {
        return;
    }

    let mut y = y;

    if y < 0 {
        length = length + y;
        y = 0;
    }

    if y + length > height {
        length = height - y;
    }

    let mut i = 0;
    let l = length * 4;

    loop {
        let y = y + (i / 4);

        let index = (x + y * width) * 4;

        buffer[index as usize + 0] = color.red;
        buffer[index as usize + 1] = color.green;
        buffer[index as usize + 2] = color.blue;
        buffer[index as usize + 3] = color.alpha;

        i += 4;

        if i >= l {
            break;
        }
    }
}
