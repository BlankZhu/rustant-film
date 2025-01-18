#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub top: u32,
    pub bottom: u32,
    pub left: u32,
    pub right: u32,
}

impl Padding {
    pub fn new(top: u32, bottom: u32, left: u32, right: u32) -> Self {
        Padding {
            top,
            bottom,
            left,
            right,
        }
    }
}
