use std::f32::consts::PI;

use std::collections::HashSet;

use crate::display::window::Graphics;

use crate::graphics::color::Color;
use crate::graphics::rectangle::Rectangle;

use super::data::{color_list, grid, ColorGroup};
use super::data::{GREY2, GREY3, GREY7, GREY8};
use super::logic::{render_graphics, update_state};

pub type Map = Rectangle;

// Ray type
#[derive(PartialEq)]
pub enum Type {
    None,
    All,
    Map,
    Surface,
}

// Ray orientation
#[derive(PartialEq)]
pub enum Orientation {
    None,
    Left,
    Right,
    Up,
    Down,
}

// Collision data
pub struct Collision {
    pub id: i32,
    pub distance: f32,
    pub orientation: Orientation,
}

impl Collision {
    pub fn new(id: i32, distance: f32, orientation: Orientation) -> Self {
        Self {
            id,
            distance,
            orientation,
        }
    }
}

// Ray data
pub struct Ray {
    pub id: i32,
    pub angle: f32,
    pub height: f32,
    pub distance: f32,
    pub orientation: Orientation,
}

impl Ray {
    pub fn new(id: i32, angle: f32, height: f32, distance: f32, orientation: Orientation) -> Self {
        Self {
            id,
            angle,
            height,
            distance,
            orientation,
        }
    }
}

// State setup
#[derive(Default)]
pub struct State {
    pub conf: Configuration, // Game
    pub env: Environment,    // Surface
    pub sub: Subject,        // Player
    pub colors: Colors,      // Colors
}

// Game setup
#[derive(Default)]
pub struct Configuration {
    pub fov: f32,              // Field of view
    pub arc: f32,              // Arc delta
    pub ratio: u32,            // Ray ratio
    pub resolution: u32,       // Ray resolution
    pub time: f32,             // Frame total time
    pub delta: f32,            // Frame delta time
    pub fps: f32,              // Frames per second
    pub left: bool,            // Key left
    pub right: bool,           // Key right
    pub up: bool,              // Key up
    pub down: bool,            // Key down
    pub grid: Vec<Vec<u32>>,   // Map grid data
    pub keys: HashSet<String>, // Keys pressed
}

// Surface properties
#[derive(Default)]
pub struct Environment {
    pub x: u32,      // Surface x
    pub y: u32,      // Surface y
    pub width: u32,  // Surface width
    pub height: u32, // Surface height
    pub unit: f32,   // Map unit size
    pub map: Map,    // Map coordinates and dimensions
}

// Player properties
#[derive(Default)]
pub struct Subject {
    pub x: f32,                   // Player x
    pub y: f32,                   // Player y
    pub radius: f32,              // Player radius
    pub sector: f32,              // Player fov sector start
    pub direction: f32,           // Player direction
    pub rotate_amount: f32,       // Rotate amount
    pub translate_amount: f32,    // Translate amount
    pub rotate_direction: f32,    // Rotate direction
    pub translate_direction: f32, // Translate direction
}

// Colors
#[derive(Default)]
pub struct Colors {
    pub ceiling: Color,
    pub floor: Color,
    pub player: Color,
    pub map: Color,
    pub list: [ColorGroup; 10],
}

impl State {
    pub fn new() -> Self {
        Self {
            conf: Configuration::new(),
            env: Environment::new(),
            sub: Subject::new(),
            colors: Colors::new(),
        }
    }
    pub fn init(&mut self) {
        self.conf.resolution = self.env.width / self.conf.ratio;
        self.conf.arc = self.conf.fov / self.conf.resolution as f32;
    }
    pub fn build() -> Self {
        let mut state = Self::new();
        state.init();
        state
    }
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            fov: PI / 3.0,
            arc: 0.0,
            ratio: 1,
            resolution: 960,
            time: 0.0,
            delta: 0.0,
            fps: 0.0,
            left: false,
            right: false,
            up: false,
            down: false,
            grid: grid(),
            keys: HashSet::new(),
        }
    }
}

impl Environment {
    pub fn new() -> Self {
        let map = Map::new(640.0, 480.0, 320.0, 240.0, true);
        Self {
            x: 0,
            y: 0,
            width: 960,
            height: 720,
            unit: 10.0,
            map,
        }
    }
}

impl Subject {
    pub fn new() -> Self {
        Self {
            x: 120.0,
            y: 40.0,
            radius: 2.0,
            sector: 0.0,
            direction: PI / 6.0,
            rotate_amount: PI * 0.0125,
            translate_amount: 1.0,
            rotate_direction: 0.0,
            translate_direction: 0.0,
        }
    }
}

impl Colors {
    pub fn new() -> Self {
        Self {
            ceiling: GREY3,
            floor: GREY7,
            player: GREY2,
            map: GREY8,
            list: color_list(),
        }
    }
}

impl Graphics for State {
    fn input(&mut self, active: bool, key: &str) {
        if active {
            self.conf.keys.insert(key.into());
        } else {
            self.conf.keys.remove(key);
        }
        match key {
            "s" => self.conf.left = active,
            "f" => self.conf.right = active,
            "e" => self.conf.up = active,
            "d" => self.conf.down = active,
            _ => {}
        }
    }
    fn update(&mut self, time: f32, delta: f32, fps: f32) {
        update_state(self, time, delta, fps);
    }
    fn render(&mut self, buffer: &mut [u8], width: u32, height: u32) {
        render_graphics(self, buffer, width, height);
    }
}
