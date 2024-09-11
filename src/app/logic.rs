use super::detection::{collision, constrain};
use super::graphics::{draw_map, draw_map_rays, draw_subject, draw_surface, draw_surface_rays};
use super::state::{Configuration, Environment, Map, State, Subject};

// Handle keyboard input
pub fn handle_input(state: &mut State) {
    state.sub.rotate_direction = 0.0;
    state.sub.translate_direction = 0.0;

    if state.conf.left && !state.conf.right {
        state.sub.rotate_direction = -1.0;
    }

    if state.conf.right && !state.conf.left {
        state.sub.rotate_direction = 1.0;
    }

    if state.conf.up && !state.conf.down {
        state.sub.translate_direction = 1.0;
    }

    if state.conf.down && !state.conf.up {
        state.sub.translate_direction = -1.0;
    }
}

// Update application state
pub fn update_state(state: &mut State, time: f32, delta: f32, fps: f32) {
    state.conf.time = time;
    state.conf.delta = delta;
    state.conf.fps = fps;

    handle_input(state);

    let Configuration { fov, ref grid, .. } = state.conf;

    let Environment { unit, map, .. } = state.env;

    let Map { width, height, .. } = map;

    let Subject {
        x,
        y,
        direction,
        translate_amount,
        translate_direction,
        rotate_amount,
        rotate_direction,
        ..
    } = state.sub;

    let rotation = rotate_direction * rotate_amount;
    let translation = translate_direction * translate_amount;

    state.sub.direction = constrain(direction + rotation);
    state.sub.sector = constrain(direction - fov / 2.0);

    let x = x + translation * direction.cos();
    let y = y + translation * direction.sin();

    let id = collision(x, y, width, height, unit, grid);

    if id == 0 {
        state.sub.x = x;
        state.sub.y = y;
    }
}

// Render graphics
pub fn render_graphics(state: &mut State, buffer: &mut [u8], width: u32, height: u32) {
    draw_surface(state, buffer, width, height);
    draw_surface_rays(state, buffer, width, height);
    draw_map(state, buffer, width, height);
    draw_map_rays(state, buffer, width, height);
    draw_subject(state, buffer, width, height);
}
