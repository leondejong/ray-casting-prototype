use ray_casting_prototype::display::run;

use ray_casting_prototype::app::data::{FILTER, HEIGHT, RESIZABLE, SCALE, TITLE, WIDTH};
use ray_casting_prototype::app::state::State;

fn main() {
    let state = State::build();
    run(WIDTH, HEIGHT, SCALE, RESIZABLE, FILTER, TITLE.into(), state);
}
