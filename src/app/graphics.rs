use crate::canvas;

use crate::graphics::color::Color;
use crate::graphics::line::Line;
use crate::graphics::rectangle::Rectangle;

use super::detection::cast_ray;
use super::state::{Configuration, Environment, Map, Orientation, Ray, State, Subject, Type};

// Render rectangle
pub fn draw_rectangle(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    color: &Color,
) {
    let rectangle = Rectangle::new(x, y, w, h, true);
    canvas::rectangle::render(buffer, width, height, &rectangle, color);
}

// Render scene background
pub fn draw_surface(state: &mut State, buffer: &mut [u8], width: u32, height: u32) {
    let Environment {
        x,
        y,
        width: w,
        height: h,
        ..
    } = state.env;

    draw_rectangle(
        buffer,
        width,
        height,
        x as f32,
        y as f32,
        w as f32,
        (h / 2) as f32,
        &state.colors.ceiling,
    );

    draw_rectangle(
        buffer,
        width,
        height,
        x as f32,
        (y + height / 2) as f32,
        w as f32,
        (h / 2) as f32,
        &state.colors.floor,
    );
}

// Generate and render map
pub fn draw_map(state: &mut State, buffer: &mut [u8], width: u32, height: u32) {
    let background = state.colors.map;

    let grid = &state.conf.grid;

    let Environment { unit, map, .. } = state.env;

    let Map {
        x: map_x,
        y: map_y,
        width: map_width,
        height: map_height,
        ..
    } = map;

    draw_rectangle(
        buffer,
        width,
        height,
        map_x,
        map_y,
        map_width,
        map_height,
        &background,
    );

    for (grid_y, row) in grid.iter().enumerate() {
        for (grid_x, id) in row.iter().enumerate() {
            let x = map_x + unit * grid_x as f32;
            let y = map_y + unit * grid_y as f32;

            let color = state.colors.list[*id as usize].base;

            if *id > 0 {
                draw_rectangle(buffer, width, height, x, y, unit, unit, &color);
            }
        }
    }
}

// Generate and render player
pub fn draw_subject(state: &mut State, buffer: &mut [u8], width: u32, height: u32) {
    let color = state.colors.player;

    let Map {
        x: map_x, y: map_y, ..
    } = state.env.map;

    let Subject {
        x: sub_x,
        y: sub_y,
        radius,
        ..
    } = state.sub;

    let x = map_x + sub_x - radius;
    let y = map_y + sub_y - radius;
    let r = 2.0 * radius;

    draw_rectangle(buffer, width, height, x, y, r, r, &color);
}

// Generate and render map ray
pub fn draw_map_ray(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    length: f32,
    map_x: f32,
    map_y: f32,
    sub_x: f32,
    sub_y: f32,
    angle: f32,
    color: &Color,
) {
    let x0 = map_x + sub_x;
    let y0 = map_y + sub_y;
    let x1 = x0 + length * angle.cos();
    let y1 = y0 + length * angle.sin();

    let line = Line::new(x0, y0, x1, y1);
    canvas::line::render(buffer, width, height, &line, color);
}

// Generate and render scene ray
pub fn draw_surface_ray(
    buffer: &mut [u8],
    width: u32,
    height: u32,
    env_x: f32,
    env_y: f32,
    length: f32,
    max: f32,
    index: u32,
    ratio: u32,
    base: &Color,
    shade: &Color,
    orientation: Orientation,
) {
    let ray_x = (env_x + (index * ratio) as f32) as i32;
    let ray_y = (env_y + (max - length) / 2.0) as i32;

    let horizontal = orientation == Orientation::Left || orientation == Orientation::Right;
    let color = if horizontal { base } else { shade };

    canvas::line::vertical(buffer, width, height, ray_x, ray_y, length as u32, color);
}

// Generate and render rays
pub fn draw_rays(state: &mut State, buffer: &mut [u8], width: u32, height: u32, kind: Type) {
    let Configuration {
        ref grid,
        arc,
        ratio,
        resolution,
        ..
    } = state.conf;

    let Environment {
        x: env_x,
        y: env_y,
        height: max,
        unit,
        map,
        ..
    } = state.env;

    let Map {
        x: map_x,
        y: map_y,
        width: map_width,
        height: map_height,
        ..
    } = map;

    let Subject {
        x: sub_x,
        y: sub_y,
        sector,
        direction,
        ..
    } = state.sub;

    for index in 0..resolution {
        let ray = cast_ray(
            sub_x,
            sub_y,
            map_width,
            map_height,
            direction,
            sector,
            arc,
            index as f32,
            max as f32,
            unit,
            grid,
        );

        let Ray {
            id,
            angle,
            height: ray_height,
            distance,
            orientation,
        } = ray;

        if id > 0 && (kind == Type::All || kind == Type::Map) {
            let color = state.colors.list[id as usize].tint;

            draw_map_ray(
                buffer, width, height, distance, map_x, map_y, sub_x, sub_y, angle, &color,
            );
        }

        if id > 0 && (kind == Type::All || kind == Type::Surface) {
            let color_base = state.colors.list[id as usize].base;
            let color_shade = state.colors.list[id as usize].shade;

            draw_surface_ray(
                buffer,
                width,
                height,
                env_x as f32,
                env_y as f32,
                ray_height,
                max as f32,
                index,
                ratio,
                &color_base,
                &color_shade,
                orientation,
            );
        }
    }
}

// Generate and render map rays
pub fn draw_map_rays(state: &mut State, buffer: &mut [u8], width: u32, height: u32) {
    draw_rays(state, buffer, width, height, Type::Map);
}

// Generate and render scene rays
pub fn draw_surface_rays(state: &mut State, buffer: &mut [u8], width: u32, height: u32) {
    draw_rays(state, buffer, width, height, Type::Surface);
}
