use ab_glyph::{FontVec, PxScale};
use image::{
    imageops::{resize, FilterType},
    GenericImage, Rgb, RgbImage,
};
use log::debug;

use crate::{
    entity::{ExifInfo, Padding, Position},
    film::{
        paint::{
            add_padding, add_text,
            constant::{BLACK, GOLDEN_RATIO, GRAY, WHITE},
            create_canvas, get_text_scaled_length,
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
    pub fn new(
        cache: LogoCache,
        font: FontVec,
        sub_font: Option<FontVec>,
        main_position: Position,
        pad_around: bool,
    ) -> Self {
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

    pub fn get_lines(&self, exif_info: &ExifInfo) -> Vec<String> {
        let camera_model = exif_info
            .camera_model
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_string();
        let lens_model = exif_info
            .lens_model
            .as_deref()
            .unwrap_or("")
            .trim()
            .to_string();

        let focal_length = exif_info
            .focal_length
            .as_deref()
            .unwrap_or("")
            .chars()
            .filter(|&c| c != ' ')
            .collect::<String>()
            .trim()
            .to_string();
        let apreture = exif_info
            .aperture
            .as_deref()
            .unwrap_or("")
            .chars()
            .filter(|&c| c != ' ')
            .collect::<String>()
            .trim()
            .to_string();
        let exposure = exif_info
            .exposure_time
            .as_deref()
            .unwrap_or("")
            .chars()
            .filter(|&c| c != ' ')
            .collect::<String>()
            .trim()
            .to_string();
        let mut iso: String = exif_info
            .iso
            .as_deref()
            .unwrap_or("")
            .chars()
            .filter(|&c| c != ' ')
            .collect::<String>()
            .trim()
            .to_string();
        if !iso.is_empty() {
            iso = format!("ISO{}", iso);
        }
        let mut detail = vec![focal_length, apreture, exposure, iso];
        detail.retain(|s| !s.is_empty());
        let detail = detail.join(" ");

        let artist = exif_info.artist.as_deref().unwrap_or("").trim().to_string();
        let copyright: String = match artist.is_empty() {
            true => "".to_string(),
            false => format!("by @{}", artist),
        };

        vec![camera_model, lens_model, detail, copyright]
    }

    pub fn create_content_canvas(
        &self,
        exif_info: &ExifInfo,
        font: &FontVec,
        sub_font: &Option<FontVec>,
        base_scale: &PxScale,
        background: Rgb<u8>,
    ) -> Result<RgbImage, Box<dyn std::error::Error>> {
        // get the lines
        let lines = self.get_lines(exif_info);

        // calculate the min width & height for the lines
        let lines_height = (lines.len() + 1) as u32 * base_scale.y as u32;  // +1 for a extra line space to tell logo & lines apart
        let lines_width = lines
            .iter()
            .map(|line| get_text_scaled_length(line, font, base_scale))
            .max()
            .unwrap_or(0);

        // get the logo
        let logo_name = exif_info.camera_maker.as_deref().unwrap_or("");
        let logo = self.cache.get(logo_name);

        let mut logo_height: u32 = 0;
        let mut logo_width: u32 = 0;
        if logo.is_some() {
            let logo = logo.unwrap();
            let (logo_ori_width, logo_ori_height) = logo.dimensions();
            let mut logo_new_height = base_scale.y * 1.5;
            if (logo_ori_width as f32 / logo_ori_height as f32) <= 1.5 {
                logo_new_height = base_scale.y * 3.0;
            }
            let logo_new_width = logo_new_height * logo_ori_width as f32 / logo_ori_height as f32;
            logo_height = logo_new_height as u32;
            logo_width = logo_new_width as u32;
        }

        // create a canvas that holds the lines & logo
        let canvas_width = std::cmp::max(lines_width, logo_width);
        let canvas_height = lines_height + logo_height;
        debug!("content canvas width: {}, height: {}", canvas_width, canvas_height);
        let mut canvas = create_canvas(canvas_width, canvas_height, background);

        // print logo
        let mut curr_y: u32 = 0;
        if logo.is_some() {
            debug!("paint logo in at top");
            let logo: RgbImage =
                resize(logo.unwrap(), logo_width, logo_height, FilterType::Lanczos3);
            let x = (canvas_width - logo_width) / 2;
            canvas.copy_from(&logo, x, curr_y)?;
            curr_y += logo_height;
        }

        // give some space between logo & following lines
        curr_y += base_scale.y as u32;

        // print lines
        debug!("paint lines one by one");
        let mut index = 0;
        for line in lines {
            if line.is_empty() {
                continue;
            }

            let color = match index {
                0 => &BLACK,
                _ => &GRAY,
            };
            let font_to_use = match index {
                0 => font,
                _ => sub_font.as_ref().unwrap_or(font),
            };

            let line_width = get_text_scaled_length(&line, font_to_use, base_scale);
            let x = (canvas_width - line_width) / 2;
            debug!("paint line: {} at x: {}, y: {}", line, x, curr_y);
            add_text(
                &mut canvas,
                x,
                curr_y,
                &line,
                base_scale,
                font_to_use,
                color,
            );

            curr_y += base_scale.y as u32;
            index += 1;
        }

        Ok(canvas)
    }

    pub fn create_main_content_canvas(
        &self,
        height: u32,
        exif_info: &ExifInfo,
        font_scale: &PxScale,
        padding: &Padding,
        background: Rgb<u8>,
    ) -> Result<RgbImage, Box<dyn std::error::Error>> {
        // get the content canvas
        let content = self.create_content_canvas(
            exif_info,
            &self.font,
            &self.sub_font,
            font_scale,
            background,
        )?;
        let width = content.width() + padding.left + padding.right;

        // get the main canvas
        let mut canvas = create_canvas(width, height, background);
        canvas.copy_from(&content, padding.left, (height - content.height()) / 2)?;

        Ok(canvas)
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
        let mut trivial_padding: u32 = 0;
        if self.pad_around {
            trivial_padding = standard_padding;
        }

        // prepare font
        let font_size = standard_padding as f32 * 0.5;
        let font_scale = PxScale {
            x: font_size,
            y: font_size,
        };

        // create a new main content canvas
        let main_content_canvas_padding =
            Padding::new(0, 0, standard_padding, standard_padding);
        let main_content_canvas = self.create_main_content_canvas(
            ori_height + trivial_padding * 2,
            exif_info,
            &font_scale,
            &main_content_canvas_padding,
            WHITE,
        )?;

        // setup padding around the image
        let mut padding = Padding::new(
            trivial_padding,
            trivial_padding,
            trivial_padding,
            main_content_canvas.width(),
        );
        if self.main_position == Position::LEFT {
            padding = Padding::new(
                trivial_padding,
                trivial_padding,
                main_content_canvas.width(),
                trivial_padding,
            );
        }
        add_padding(image, &padding, &WHITE)?;

        // move the main content into image's padding

        if self.main_position == Position::LEFT {
            image.copy_from(&main_content_canvas, 0, 0)?;
        } else {
            image.copy_from(&main_content_canvas, ori_width + padding.left, 0)?;
        }

        Ok(())
    }
}
