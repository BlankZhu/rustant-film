use crate::film::paint::constant;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Position {
    TOP,
    MIDDLE,
    BOTTOM,
    LEFT,
    RIGHT,
}

pub fn from_str(s: &str) -> Option<Position> {
    match s.to_ascii_lowercase().as_str() {
        constant::POSITION_MIDDLE | constant::POSITION_MIDDLE_SHORT => Some(Position::MIDDLE),
        constant::POSITION_TOP | constant::POSITION_TOP_SHORT => Some(Position::TOP),
        constant::POSITION_BOTTOM | constant::POSITION_BOTTOM_SHORT => Some(Position::BOTTOM),
        constant::POSITION_LEFT | constant::POSITION_LEFT_SHORT => Some(Position::LEFT),
        constant::POSITION_RIGHT | constant::POSITION_RIGHT_SHORT => Some(Position::RIGHT),
        _ => None,
    }
}