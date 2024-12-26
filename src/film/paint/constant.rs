use image::Rgb;

pub const GOLDEN_RATIO: f32 = 1.61803398874989484820;

pub const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
pub const GRAY: Rgb<u8> = Rgb([125, 127, 124]);
pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

pub const HORIZONTAL_PAINTER: &str = "horizontal";
pub const VERTICAL_PAINTER: &str = "vertical";

pub const POSITION_TOP: &str = "top";
pub const POSITION_MIDDLE: &str = "middle";
pub const POSITION_BOTTOM: &str = "bottom";
pub const POSITION_LEFT: &str = "left";
pub const POSITION_RIGHT: &str = "right";
pub const POSITION_TOP_SHORT: &str = "t";
pub const POSITION_MIDDLE_SHORT: &str = "m";
pub const POSITION_BOTTOM_SHORT: &str = "b";
pub const POSITION_LEFT_SHORT: &str = "l";
pub const POSITION_RIGHT_SHORT: &str = "r";

// should remove the following constants
pub const NORMAL_PAINTER: &str = "normal";
pub const BOTTOM_PAINTER: &str = "bottom";
pub const RIGHT_PAINTER: &str = "right";