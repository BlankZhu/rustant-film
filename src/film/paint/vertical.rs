use ab_glyph::FontVec;

use crate::{entity::Position, film::LogoCache};

use super::Painter;

pub struct VerticalPainter {
    cache: LogoCache,
    font: FontVec,
    main_position: Position,
    content_position: Position,
    pad_around: bool,
}

impl VerticalPainter {
    pub fn new(cache: LogoCache, font: FontVec) -> Self {
        VerticalPainter {
            cache,
            font,
            main_position: Position::RIGHT,
            content_position: Position::BOTTOM,
            pad_around: true,
        }
    }
}

impl Painter for VerticalPainter {
    fn paint(&self, image: &mut image::RgbImage, exif_info: &crate::entity::ExifInfo) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}