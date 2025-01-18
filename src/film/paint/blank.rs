use log::debug;

use crate::{
    entity::{ExifInfo, Padding},
    film::paint::{
        add_padding,
        constant::{GOLDEN_RATIO, WHITE},
    },
};

use super::Painter;

pub struct BlankPainter {
    pad_around: bool,
}

impl BlankPainter {
    pub fn new(pad_around: bool) -> Self {
        BlankPainter { pad_around }
    }

    pub fn new_normal() -> Self {
        BlankPainter { pad_around: true }
    }
}

impl Painter for BlankPainter {
    fn paint(
        &self,
        image: &mut image::RgbImage,
        _exif_info: &ExifInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // get concrete values
        let (ori_width, ori_height) = image.dimensions();
        let long_side = std::cmp::max(ori_width, ori_height);
        debug!("origin image width: {}, height: {}", ori_width, ori_height);

        // setup padding size
        let padding_ratio_0 = 1.0 / GOLDEN_RATIO / 16.0;
        let standard_padding = (long_side as f32 * padding_ratio_0) as u32; // twice of the scaled font height
        let main_padding = standard_padding * 3; // 3 times for main
        let mut trivial_padding: u32 = 0;
        if self.pad_around {
            trivial_padding = standard_padding; // 1 times for other
        }
        let padding = Padding::new(
            trivial_padding,
            main_padding,
            trivial_padding,
            trivial_padding,
        );

        add_padding(image, &padding, &WHITE)?;
        Ok(())
    }
}
