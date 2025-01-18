use std::sync::Arc;

use ab_glyph::{Font, FontVec, PxScale, ScaleFont};
use image::{
    imageops::{resize, FilterType},
    GenericImage, Rgb, RgbImage,
};
use log::debug;

use crate::{
    entity::{ExifInfo, Padding, Position},
    film::{
        paint::{
            add_padding, add_text, add_vertical_line,
            constant::{BLACK, GOLDEN_RATIO, GRAY, WHITE},
            create_canvas, get_text_scaled_length, Painter,
        },
        LogoCache,
    },
};

pub struct TriangularPainter {
    cache: Arc<LogoCache>,
    font: Arc<FontVec>,
    sub_font: Arc<Option<FontVec>>,
    main_position: Position,
    pad_around: bool,
}

impl TriangularPainter {
    pub fn new(
        cache: Arc<LogoCache>,
        font: Arc<FontVec>,
        sub_font: Arc<Option<FontVec>>,
        main_position: Position,
        pad_around: bool,
    ) -> Self {
        TriangularPainter {
            cache,
            font,
            sub_font,
            main_position,
            pad_around,
        }
    }

    pub fn new_normal(cache: Arc<LogoCache>, font: Arc<FontVec>) -> Self {
        TriangularPainter {
            cache,
            font,
            sub_font: None.into(),
            main_position: Position::BOTTOM,
            pad_around: true,
        }
    }

    fn get_lens_model_text(&self, exif_info: &ExifInfo) -> String {
        exif_info.lens_model.clone().unwrap_or("".to_string())
    }

    fn get_camera_model_text(&self, exif_info: &ExifInfo) -> String {
        exif_info.camera_model.clone().unwrap_or("".to_string())
    }

    fn get_shooting_parameters_text(&self, exif_info: &ExifInfo) -> String {
        let focal_length: String = exif_info
            .focal_length
            .clone()
            .unwrap_or("".to_string())
            .chars()
            .filter(|&c| c != ' ')
            .collect();
        let apreture: String = exif_info
            .aperture
            .clone()
            .unwrap_or("".to_string())
            .chars()
            .filter(|&c| c != ' ')
            .collect();
        let exposure: String = exif_info
            .exposure_time
            .clone()
            .unwrap_or("".to_string())
            .chars()
            .filter(|&c| c != ' ')
            .collect();
        let mut iso: String = exif_info
            .iso
            .clone()
            .unwrap_or("".to_string())
            .chars()
            .filter(|&c| c != ' ')
            .collect();
        if !iso.is_empty() {
            iso = format!("{}{}", "ISO", iso);
        }

        let mut details = vec![focal_length, apreture, exposure, iso];
        details.retain(|s| !s.is_empty());
        details.join(" ")
    }

    fn get_datetime_text(&self, exif_info: &ExifInfo) -> String {
        exif_info.datetime.clone().unwrap_or("".to_string())
    }

    ///
    /// The layout of main content canvas is like:
    ///
    /// ```txt
    /// +------------------------------+
    /// |                              | - Padding-Top
    /// |   +----------------------+   |
    /// |   |                      |   |
    /// |   |     Main Content     |   |
    /// |   |                      |   |
    /// |   +----------------------+   |
    /// |                              | - Padding-Bottom
    /// +------------------------------+
    ///   |                          |  
    ///  Padding Left               Padding Right
    /// ```
    ///
    /// In side of the `main content`, it shows something like:
    ///
    /// ```txt
    /// +---------------------------------------------+
    /// |                                             |
    /// |                                             |
    /// | LensModel         Logo | ShootingParameters |
    /// | CameraModel       Logo | Datetime           |
    /// |                                             |
    /// |                                             |
    /// +---------------------------------------------+
    /// ```
    fn create_main_content_canvas(
        &self,
        width: u32,
        height: u32,
        exif_info: &ExifInfo,
        font_scale: &PxScale,
        padding: &Padding,
        background: Rgb<u8>,
    ) -> Result<RgbImage, Box<dyn std::error::Error>> {
        debug!(
            "creating main content with width {}, height {}, padding: {:?}",
            width, height, padding
        );

        // create canvas for main content
        let mut canvas = create_canvas(width, height, background);

        // print lines on the left
        debug!("paint main text to the left");
        let lens_model_text = self.get_lens_model_text(exif_info);
        let camera_model_text = self.get_camera_model_text(exif_info);
        if !lens_model_text.is_empty() || !camera_model_text.is_empty() {
            let text_canvas = self.create_text_canvas_with_emphasized_first_line(
                &vec![lens_model_text, camera_model_text],
                &self.font,
                &self.sub_font,
                font_scale,
                Position::LEFT,
                background,
            );
            canvas.copy_from(&text_canvas, padding.left, padding.top)?;
        }

        // print lines on the right
        debug!("paint main text to the right");
        let shooting_parameters_text = self.get_shooting_parameters_text(exif_info);
        let datetime_text = self.get_datetime_text(exif_info);
        let has_text_on_right = !shooting_parameters_text.is_empty() || !datetime_text.is_empty();
        let mut right_text_canvas_width: u32 = 0;
        if has_text_on_right {
            let text_canvas = self.create_text_canvas_with_emphasized_first_line(
                &vec![shooting_parameters_text, datetime_text],
                &self.font,
                &self.sub_font,
                font_scale,
                Position::RIGHT,
                background,
            );
            let (text_canvas_width, _) = text_canvas.dimensions();
            right_text_canvas_width = text_canvas_width;

            canvas.copy_from(
                &text_canvas,
                width - padding.right - right_text_canvas_width,
                padding.top,
            )?;
        }

        // print vertical delimiter if possible
        let logo_name = exif_info.camera_maker.as_deref().unwrap_or("");
        let logo = self.cache.get(logo_name);
        let has_logo_on_right = self.cache.get(logo_name).is_some();
        if has_text_on_right && has_logo_on_right {
            debug!("paint vertical delimiter to the right");
            add_vertical_line(
                &mut canvas,
                width - padding.right - right_text_canvas_width - font_scale.y as u32,
                padding.top,
                padding.top,
                std::cmp::max((font_scale.y / 32.0) as u32, 1),
                &GRAY,
                &background,
            );
        }

        // print logo
        debug!("paint logo to the right");
        if has_logo_on_right {
            let logo = logo.unwrap();
            let (logo_ori_width, logo_ori_height) = logo.dimensions();
            let mut logo_new_height = font_scale.y * 0.75;
            if (logo_ori_width as f32 / logo_ori_height as f32) <= 1.5 {
                // logo not too wide
                logo_new_height = font_scale.y * 1.75;
            }
            let logo_new_width = logo_new_height * logo_ori_width as f32 / logo_ori_height as f32;
            let logo: RgbImage = resize(
                logo,
                logo_new_width as u32,
                logo_new_height as u32,
                FilterType::Lanczos3,
            );
            debug!(
                "logo to paint has width: {}, height: {}",
                logo_new_width as u32, logo_new_height as u32
            );
            canvas.copy_from(
                &logo,
                width
                    - padding.right
                    - right_text_canvas_width
                    - (font_scale.y * 2.0) as u32
                    - logo_new_width as u32,
                padding.top + (padding.top - logo_new_height as u32) / 2,
            )?;
        }

        Ok(canvas)
    }

    fn create_text_canvas_with_emphasized_first_line(
        &self,
        lines: &Vec<String>,
        font: &FontVec,
        sub_font: &Option<FontVec>,
        scale: &PxScale,
        align: Position,
        background: Rgb<u8>,
    ) -> RgbImage {
        // quick return
        if lines.is_empty() {
            let mut ret = RgbImage::new(1, 1);
            ret.put_pixel(0, 0, background);
            return ret;
        }

        // calculate the size of canvas
        let scaled_font = font.as_scaled(*scale);
        let height = std::cmp::max(scaled_font.height() as u32 * lines.len() as u32, 1);
        let width = lines
            .iter()
            .map(|line| get_text_scaled_length(line, font, scale))
            .max()
            .unwrap_or(1);

        // create the canvas
        let mut canvas = create_canvas(width, height, background);

        // print lines on it
        let mut index: u32 = 0;
        for line in lines {
            let color = match index {
                0 => &BLACK,
                _ => &GRAY,
            };
            let font_to_use = match index {
                0 => font,
                _ => sub_font.as_ref().unwrap_or(font),
            };

            let mut x = 0;
            let y = index * (scaled_font.height() as u32);

            // adjust x position if align to right
            if align == Position::RIGHT {
                // get rendered length of current line
                let line_width = get_text_scaled_length(line, font_to_use, scale);
                x = width - line_width;
            }

            // paint the lines to canvas
            add_text(&mut canvas, x, y, &line, scale, font_to_use, color);
            index += 1;
        }

        return canvas;
    }
}

impl Painter for TriangularPainter {
    fn paint(
        &self,
        image: &mut image::RgbImage,
        exif_info: &ExifInfo,
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
        let mut padding = Padding::new(
            trivial_padding,
            main_padding,
            trivial_padding,
            trivial_padding,
        );

        // add padding
        debug!("painting paddings...");
        if self.main_position == Position::TOP {
            padding = Padding::new(
                main_padding,
                trivial_padding,
                trivial_padding,
                trivial_padding,
            );
        }
        add_padding(image, &padding, &WHITE)?;

        // prepare the font
        let font_size = standard_padding as f32 * 0.5; // for both height & width
        let font_scale = PxScale {
            x: font_size,
            y: font_size,
        };

        debug!("standard padding size: {}, main padding size: {}, trivial padding size: {}, font size: {}", standard_padding, main_padding, trivial_padding, font_size as u32);
        debug!(
            "padded image width: {}, height: {}",
            ori_width + padding.left + padding.right,
            ori_height + padding.top + padding.bottom
        );

        // create a new main content canvas
        let main_content_canvas_padding = Padding::new(
            standard_padding,
            standard_padding,
            padding.left + standard_padding / 2,
            padding.right + standard_padding / 2,
        );
        let main_content_canvas = self.create_main_content_canvas(
            ori_width + trivial_padding * 2,
            main_padding,
            exif_info,
            &font_scale,
            &main_content_canvas_padding,
            WHITE,
        )?;

        // put main content canvas back
        if self.main_position == Position::TOP {
            image.copy_from(&main_content_canvas, 0, 0)?;
        } else {
            image.copy_from(&main_content_canvas, 0, ori_height + padding.top)?;
        }

        Ok(())
    }
}
