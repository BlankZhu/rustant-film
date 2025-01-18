// math constants
pub const GOLDEN_RATIO: f32 = 1.61803398874989484820;

// color constants
pub const BLACK: image::Rgb<u8> = image::Rgb::<u8>([0, 0, 0]);
pub const GRAY: image::Rgb<u8> = image::Rgb::<u8>([125, 127, 124]);
pub const WHITE: image::Rgb<u8> = image::Rgb::<u8>([255, 255, 255]);

// position constants
pub const POSITION_MIDDLE: &str = "middle";
pub const POSITION_TOP: &str = "top";
pub const POSITION_BOTTOM: &str = "bottom";
pub const POSITION_LEFT: &str = "left";
pub const POSITION_RIGHT: &str = "right";
pub const POSITION_MIDDLE_SHORT: &str = "m";
pub const POSITION_TOP_SHORT: &str = "t";
pub const POSITION_BOTTOM_SHORT: &str = "b";
pub const POSITION_LEFT_SHORT: &str = "l";
pub const POSITION_RIGHT_SHORT: &str = "r";

// should remove the following constants
pub const TRIANGLULAR_PAINTER: &str = "triangular";
pub const BLANK_PAINTER: &str = "blank";
pub const DUEL_PAINTER: &str = "duel";
pub const DIAGONAL_PAINTER: &str = "diagonal";