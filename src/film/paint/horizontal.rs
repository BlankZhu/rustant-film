use ab_glyph::FontVec;

use crate::{entity::Position, film::LogoCache};

use super::Painter;

pub struct HorizontalPainter {
    cache: LogoCache,
    font: FontVec,
    main_position: Position,
    content_position: Position,
    pad_around: bool,
}

impl HorizontalPainter {
    pub fn new(cache: LogoCache, font: FontVec) -> Self {
        HorizontalPainter {
            cache,
            font,
            main_position: Position::RIGHT,
            content_position: Position::BOTTOM,
            pad_around: true,
        }
    }
}

impl Painter for HorizontalPainter {
    fn paint(&self, image: &mut image::RgbImage, exif_info: &crate::entity::ExifInfo) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}