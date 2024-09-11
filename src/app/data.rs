use crate::graphics::color::Color;

// Window properties
pub const WIDTH: u32 = 960;
pub const HEIGHT: u32 = 720;
pub const RESIZABLE: bool = false;
pub const FILTER: bool = false;
pub const SCALE: f32 = 1.0;
pub const TITLE: &str = "Ray Casting";

// Colors
pub const COLOR: Color = color(0, 0, 0, 0);
pub const BACKGROUND: Color = color(15, 23, 31, 255);
pub const FOREGROUND: Color = color(223, 255, 0, 255);

pub const GREY: Color = color(127, 127, 127, 1);
pub const WHITE: Color = color(255, 255, 255, 1);
pub const BLACK: Color = color(0, 0, 0, 1);

pub const MEDIUM: Color = color(127, 135, 143, 1);
pub const MEDIUM_TINT: Color = color(143, 151, 159, 1);
pub const MEDIUM_SHADE: Color = color(111, 119, 127, 1);

pub const LIGHT: Color = color(223, 231, 239, 1);
pub const LIGHT_TINT: Color = color(239, 247, 255, 1);
pub const LIGHT_SHADE: Color = color(207, 215, 223, 1);

pub const DARK: Color = color(31, 39, 47, 1);
pub const DARK_TINT: Color = color(47, 55, 63, 1);
pub const DARK_SHADE: Color = color(15, 23, 31, 1);

pub const RED: Color = color(239, 79, 119, 1);
pub const RED_TINT: Color = color(255, 105, 158, 1);
pub const RED_SHADE: Color = color(160, 53, 85, 1);

pub const ORANGE: Color = color(239, 123, 107, 1);
pub const ORANGE_TINT: Color = color(255, 164, 142, 1);
pub const ORANGE_SHADE: Color = color(160, 82, 72, 1);

pub const YELLOW: Color = color(239, 175, 127, 1);
pub const YELLOW_TINT: Color = color(255, 233, 169, 1);
pub const YELLOW_SHADE: Color = color(160, 117, 85, 1);

pub const GREEN: Color = color(95, 175, 127, 1);
pub const GREEN_TINT: Color = color(126, 233, 169, 1);
pub const GREEN_SHADE: Color = color(64, 117, 85, 1);

pub const CYAN: Color = color(0, 143, 143, 1);
pub const CYAN_SHADE: Color = color(0, 96, 96, 1);
pub const CYAN_TINT: Color = color(0, 190, 190, 1);

pub const BLUE: Color = color(0, 87, 159, 1);
pub const BLUE_TINT: Color = color(0, 116, 211, 1);
pub const BLUE_SHADE: Color = color(0, 58, 106, 1);

pub const INDIGO: Color = color(87, 63, 159, 1);
pub const INDIGO_TINT: Color = color(116, 84, 211, 1);
pub const INDIGO_SHADE: Color = color(58, 42, 106, 1);

pub const VIOLET: Color = color(159, 63, 159, 1);
pub const VIOLET_TINT: Color = color(211, 84, 211, 1);
pub const VIOLET_SHADE: Color = color(107, 42, 107, 1);

pub const GREY1: Color = color(0, 0, 0, 1);
pub const GREY2: Color = color(31, 31, 31, 1);
pub const GREY3: Color = color(63, 63, 63, 1);
pub const GREY4: Color = color(95, 95, 95, 1);
pub const GREY5: Color = color(127, 127, 127, 1);
pub const GREY6: Color = color(159, 159, 159, 1);
pub const GREY7: Color = color(191, 191, 191, 1);
pub const GREY8: Color = color(223, 223, 223, 1);
pub const GREY9: Color = color(255, 255, 255, 1);

#[derive(Default)]
pub struct ColorGroup {
    pub base: Color,
    pub tint: Color,
    pub shade: Color,
}

impl ColorGroup {
    pub fn new(base: Color, tint: Color, shade: Color) -> Self {
        Self { base, tint, shade }
    }
}

pub const fn color(red: u8, green: u8, blue: u8, alpha: u8) -> Color {
    Color {
        red,
        green,
        blue,
        alpha,
    }
}

pub fn color_list() -> [ColorGroup; 10] {
    [
        ColorGroup::new(GREY6, GREY5, GREY4),
        ColorGroup::new(MEDIUM, MEDIUM_TINT, MEDIUM_SHADE),
        ColorGroup::new(RED, RED_TINT, RED_SHADE),
        ColorGroup::new(ORANGE, ORANGE_TINT, ORANGE_SHADE),
        ColorGroup::new(YELLOW, YELLOW_TINT, YELLOW_SHADE),
        ColorGroup::new(GREEN, GREEN_TINT, GREEN_SHADE),
        ColorGroup::new(CYAN, CYAN_TINT, CYAN_SHADE),
        ColorGroup::new(BLUE, BLUE_TINT, BLUE_SHADE),
        ColorGroup::new(INDIGO, INDIGO_TINT, INDIGO_SHADE),
        ColorGroup::new(VIOLET, VIOLET_TINT, VIOLET_SHADE),
    ]
}

// Map data
#[rustfmt::skip]
pub fn grid() -> Vec<Vec<u32>> {
    vec![
        vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,8,8,8,0,0,8,8,8,0,0,0,0,0,0,0,0,0,0,2,0,2,2,0,2,2,2,0,0,1],
        vec![1,0,0,8,0,0,0,0,0,0,8,0,0,0,0,0,0,0,0,0,0,2,0,2,2,0,2,2,2,0,0,1],
        vec![1,0,0,8,0,9,0,0,9,0,8,0,0,0,0,0,0,0,0,0,0,2,0,2,2,0,2,2,2,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,3,0,3,3,0,3,3,3,0,0,1],
        vec![1,0,0,7,0,9,0,0,9,0,7,0,0,0,0,0,0,0,0,0,0,3,0,3,3,0,3,3,3,0,0,1],
        vec![1,0,0,7,0,0,0,0,0,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,7,7,7,0,0,7,7,7,0,0,0,0,0,0,0,0,0,0,4,0,4,4,0,4,4,4,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,8,0,9,0,0,3,0,2,0,0,0,0,0,0,0,0,0,0,6,0,6,0,0,5,0,5,0,0,1],
        vec![1,0,0,0,0,0,0,0,3,0,2,0,0,0,0,0,0,0,0,0,0,0,6,6,0,0,0,5,0,0,0,1],
        vec![1,0,0,9,0,8,0,0,3,0,2,0,0,0,0,0,0,0,0,0,0,6,0,6,0,0,5,5,5,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,7,0,6,0,0,5,0,4,0,0,0,0,0,0,0,0,0,0,5,5,5,0,0,6,0,6,0,0,1],
        vec![1,0,0,7,0,6,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,0,0,6,6,0,0,0,1],
        vec![1,0,0,7,0,6,0,0,4,0,5,0,0,0,0,0,0,0,0,0,0,5,0,5,0,0,6,0,6,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1],
        vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1],
    ]
}
