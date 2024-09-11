use std::f32::consts::PI;
use std::f32::INFINITY;

use super::state::{Collision, Orientation, Ray};

// Compute line length
pub fn magnitude(x0: f32, y0: f32, x1: f32, y1: f32) -> f32 {
    ((x1 - x0).powf(2.0) + (y1 - y0).powf(2.0)).sqrt()
}

// Constrain angle within 2π radians
pub fn constrain(angle: f32) -> f32 {
    let pi2 = PI * 2.0;
    let angle = angle % pi2;

    if angle <= 0.0 {
        return angle + pi2;
    }

    return angle;
}

// Get tile id at grid intersection
pub fn intersection(x: i32, y: i32, width: i32, height: i32, grid: &Vec<Vec<u32>>) -> i32 {
    if !(x >= 0 && y >= 0 && x < width && y < height) {
        return -1;
    }

    return grid[y as usize][x as usize] as i32;
}

// Check collision with grid tiles
pub fn collision(x: f32, y: f32, width: f32, height: f32, unit: f32, grid: &Vec<Vec<u32>>) -> i32 {
    let x = (x / unit).floor() as i32;
    let y = (y / unit).floor() as i32;

    let width = (width / unit).floor() as i32;
    let height = (height / unit).floor() as i32;

    return intersection(x, y, width, height, grid);
}

// Cast ray through grid
pub fn cast_ray(
    sub_x: f32,
    sub_y: f32,
    map_width: f32,
    map_height: f32,
    direction: f32,
    sector: f32,
    arc: f32,
    index: f32,
    max: f32,
    unit: f32,
    grid: &Vec<Vec<u32>>,
) -> Ray {
    let angle = constrain(sector + arc * index);

    let Collision {
        id,
        distance,
        orientation,
    } = ray_collision(sub_x, sub_y, map_width, map_height, angle, unit, grid);

    let height = ray_height(direction, angle, distance, max, unit);

    return Ray::new(id, angle, height, distance, orientation);
}

// Compute ray height
fn ray_height(direction: f32, angle: f32, distance: f32, max: f32, unit: f32) -> f32 {
    let ratio = (direction - angle).cos();
    let length = (distance / unit) * ratio;
    let height = max / length;

    if height < max {
        return height;
    }

    return max;
}

// Check ray collision within map
fn ray_collision(
    sub_x: f32,
    sub_y: f32,
    map_width: f32,
    map_height: f32,
    angle: f32,
    unit: f32,
    grid: &Vec<Vec<u32>>,
) -> Collision {
    let vertical = collision_vertical(sub_x, sub_y, map_width, map_height, angle, unit, grid);
    let horizontal = collision_horizontal(sub_x, sub_y, map_width, map_height, angle, unit, grid);

    return if vertical.distance > horizontal.distance {
        horizontal
    } else {
        vertical
    };
}

// Check horizontal collision
fn collision_horizontal(
    sub_x: f32,
    sub_y: f32,
    map_width: f32,
    map_height: f32,
    angle: f32,
    unit: f32,
    grid: &Vec<Vec<u32>>,
) -> Collision {
    let tan = angle.tan();
    let right = angle > (PI * 3.0) / 2.0 || angle < PI / 2.0;

    let delta_x = unit;
    let delta_y = unit * tan;

    let unit_x = sub_x / unit;
    let grid_x = if right { unit_x.ceil() } else { unit_x.floor() };

    let map_x = grid_x * unit;
    let map_y = sub_y + (map_x - sub_x) * tan;

    if right {
        return collision_right(
            sub_x, sub_y, delta_x, delta_y, map_x, map_y, map_width, map_height, unit, grid,
        );
    } else {
        return collision_left(
            sub_x, sub_y, delta_x, delta_y, map_x, map_y, map_width, map_height, unit, grid,
        );
    }
}

// Check vertical collision
fn collision_vertical(
    sub_x: f32,
    sub_y: f32,
    map_width: f32,
    map_height: f32,
    angle: f32,
    unit: f32,
    grid: &Vec<Vec<u32>>,
) -> Collision {
    let tan = angle.tan();
    let top = angle > 0.0 && angle < PI;

    let delta_y = unit;
    let delta_x = unit / tan;

    let unit_y = sub_y / unit;
    let grid_y = if top { unit_y.ceil() } else { unit_y.floor() };

    let map_y = grid_y * unit;
    let map_x = sub_x - (sub_y - map_y) / tan;

    if top {
        return collision_top(
            sub_x, sub_y, delta_x, delta_y, map_x, map_y, map_width, map_height, unit, grid,
        );
    } else {
        return collision_bottom(
            sub_x, sub_y, delta_x, delta_y, map_x, map_y, map_width, map_height, unit, grid,
        );
    }
}

// Check left collision
fn collision_left(
    sub_x: f32,
    sub_y: f32,
    delta_x: f32,
    delta_y: f32,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    unit: f32,
    grid: &Vec<Vec<u32>>,
) -> Collision {
    let mut index = 0.0;

    loop {
        let tile_x = map_x - delta_x * index;
        let tile_y = map_y - delta_y * index;

        let id = collision(tile_x - unit, tile_y, map_width, map_height, unit, &grid);

        if id > 0 {
            let distance = magnitude(sub_x, sub_y, tile_x, tile_y);
            return Collision::new(id, distance, Orientation::Left);
        }

        index += 1.0;

        if map_x - delta_x * index < 0.0 {
            break;
        }
    }

    return Collision::new(-1, INFINITY, Orientation::None);
}

// Check right collision
fn collision_right(
    sub_x: f32,
    sub_y: f32,
    delta_x: f32,
    delta_y: f32,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    unit: f32,
    grid: &Vec<Vec<u32>>,
) -> Collision {
    let mut index = 0.0;

    loop {
        let tile_x = map_x + delta_x * index;
        let tile_y = map_y + delta_y * index;

        let id = collision(tile_x, tile_y, map_width, map_height, unit, &grid);

        if id > 0 {
            let distance = magnitude(sub_x, sub_y, tile_x, tile_y);
            return Collision::new(id, distance, Orientation::Right);
        }

        index += 1.0;

        if map_x + delta_x * index > map_width {
            break;
        }
    }

    return Collision::new(-1, INFINITY, Orientation::None);
}

// Check top collision
fn collision_top(
    sub_x: f32,
    sub_y: f32,
    delta_x: f32,
    delta_y: f32,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    unit: f32,
    grid: &Vec<Vec<u32>>,
) -> Collision {
    let mut index = 0.0;

    loop {
        let tile_x = map_x + delta_x * index;
        let tile_y = map_y + delta_y * index;

        let id = collision(tile_x, tile_y, map_width, map_height, unit, &grid);

        if id > 0 {
            let distance = magnitude(sub_x, sub_y, tile_x, tile_y);
            return Collision::new(id, distance, Orientation::Up);
        }

        index += 1.0;

        if map_y + delta_y * index > map_height {
            break;
        }
    }

    return Collision::new(-1, INFINITY, Orientation::None);
}

// Check bottom collision
fn collision_bottom(
    sub_x: f32,
    sub_y: f32,
    delta_x: f32,
    delta_y: f32,
    map_x: f32,
    map_y: f32,
    map_width: f32,
    map_height: f32,
    unit: f32,
    grid: &Vec<Vec<u32>>,
) -> Collision {
    let mut index = 0.0;

    loop {
        let tile_x = map_x - delta_x * index;
        let tile_y = map_y - delta_y * index;

        let id = collision(tile_x, tile_y - unit, map_width, map_height, unit, &grid);

        if id > 0 {
            let distance = magnitude(sub_x, sub_y, tile_x, tile_y);
            return Collision::new(id, distance, Orientation::Down);
        }

        index += 1.0;

        if map_y - delta_y * index < 0.0 {
            break;
        }
    }

    return Collision::new(-1, INFINITY, Orientation::None);
}
