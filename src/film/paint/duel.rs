use ab_glyph::{FontVec, PxScale};
use image::{GenericImage, Rgb, RgbImage};
use log::debug;

use crate::{
    entity::{ExifInfo, Padding, Position},
    film::{
        paint::{
            add_padding,
            constant::{GOLDEN_RATIO, WHITE},
        },
        LogoCache,
    },
};

use super::Painter;

/// actually, a flow (L-R) painter
pub struct DuelPainter {
    cache: LogoCache,
    font: FontVec,
    sub_font: Option<FontVec>,
    main_position: Position,
    pad_around: bool,
}

impl DuelPainter {
    pub fn new(cache: LogoCache, font: FontVec, sub_font: Option<FontVec>, main_position: Position, pad_around: bool) -> Self {
        DuelPainter {
            cache,
            font,
            sub_font,
            main_position,
            pad_around,
        }
    }

    pub fn new_normal(cache: LogoCache, font: FontVec) -> Self {
        DuelPainter {
            cache,
            font,
            sub_font: None,
            main_position: Position::RIGHT,
            pad_around: false,
        }
    }

    pub fn get_lines(exif_info: &ExifInfo) -> String {

        
        todo!()
    }

    pub fn create_main_content_canvas(
        &self,
        width: u32,
        height: u32,
        exif_info: &ExifInfo,
        font_scale: &PxScale,
        padding: &Padding,
        background: Rgb<u8>,
    ) -> RgbImage {
        todo!()
    }
}

impl Painter for DuelPainter {
    fn paint(
        &self,
        image: &mut image::RgbImage,
        exif_info: &crate::entity::ExifInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (ori_width, ori_height) = image.dimensions();
        let long_side = std::cmp::max(ori_width, ori_height);
        debug!("origin image width: {}, height: {}", ori_width, ori_height);

        // setup padding related variables
        let standard_padding_ratio = 1.0 / GOLDEN_RATIO / 16.0;
        let standard_padding = (long_side as f32 * standard_padding_ratio) as u32; // twice of the scaled font height
        let main_padding = standard_padding * 4; // 4 times for main content
        let mut trivial_padding: u32 = 0;
        if self.pad_around {
            trivial_padding = standard_padding;
        }
        let mut padding = Padding::new(
            trivial_padding,
            trivial_padding,
            trivial_padding,
            main_padding,
        );
        if self.main_position == Position::LEFT {
            padding = Padding::new(
                trivial_padding,
                trivial_padding,
                main_padding,
                trivial_padding,
            );
        }
        add_padding(image, &padding, &WHITE)?;

        // prepare font
        let font_size = standard_padding as f32 * 0.5;
        let font_scale = PxScale {
            x: font_size,
            y: font_size,
        };

        // fixme: get all the lines & logo and calculate a minimal canvas that holds them

        // create a new main content canvas
        let main_content_canvas_padding =
            Padding::new(0, 0, standard_padding / 2, standard_padding / 2);
        let main_content_canvas = self.create_main_content_canvas(
            main_padding,
            ori_height + padding.top + padding.bottom,
            exif_info,
            &font_scale,
            &padding,
            WHITE,
        );

        if self.main_position == Position::LEFT {
            image.copy_from(&main_content_canvas, 0, 0)?;
        } else {
            image.copy_from(&main_content_canvas, ori_width + padding.left, 0)?;
        }

        todo!()
    }
}
