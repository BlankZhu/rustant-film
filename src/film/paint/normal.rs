use ab_glyph::{FontVec, PxScale};
use image::{
    imageops::{resize, FilterType},
    GenericImage, ImageBuffer, Rgb,
};
use log::info;

use crate::film::LogoCache;

use super::{
    add_padding, add_text, add_vertical_line, calculate_text_width,
    constant::{BLACK, GRAY, WHITE},
    Painter,
};

pub struct NormalPainter {
    cache: LogoCache,
    font: FontVec,
}

impl NormalPainter {
    pub fn new(cache: LogoCache, font: FontVec) -> Self {
        NormalPainter { cache, font }
    }
}

impl Painter for NormalPainter {
    fn paint(
        &self,
        image: &mut image::RgbImage,
        exif_info: &crate::entity::ExifInfo,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (width, height) = image.dimensions();
        let standard_padding_ratio = (1.0 / 1.618033988749) / 16.0;
        let padding_base = std::cmp::max(width, height);
        let bottom_padding = (padding_base as f64 * standard_padding_ratio * 2.0) as u32;
        let standard_padding = (padding_base as f64 * standard_padding_ratio) as u32;

        // add padding
        info!("plotting paddings...");
        add_padding(
            image,
            standard_padding,
            bottom_padding,
            standard_padding,
            standard_padding,
            &WHITE,
        )?;

        let font_width = standard_padding as f32 * 0.5;
        let font_height = standard_padding as f32 * 0.5;
        let font_scale = PxScale {
            x: font_width,
            y: font_height,
        };

        // add lens text at bottom:left
        info!("plotting lens model text...");
        let lens_model_text = exif_info.lens_model.as_deref().unwrap_or("");
        add_text(
            image,
            standard_padding + (standard_padding as f32 * 0.5) as u32,
            height + (standard_padding as f32 * 1.5) as u32,
            lens_model_text,
            &font_scale,
            &self.font,
            &BLACK,
        );

        // add camera text at bottom:left
        info!("plotting camera model text...");
        let camera_model_text = exif_info.camera_model.as_deref().unwrap_or("");
        add_text(
            image,
            standard_padding + (standard_padding as f32 * 0.5) as u32,
            height + (standard_padding as f32 * 1.5) as u32 + font_height as u32,
            camera_model_text,
            &font_scale,
            &self.font,
            &GRAY,
        );

        // setup detailed info 1 at bottom:right
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
        let detail_text_1 = details.join(" ");
        let detail_1_width: f32 = calculate_text_width(&detail_text_1, &self.font, &font_scale);

        // add detail info 2 at bottom:right
        let datetime = exif_info.datetime.as_deref().unwrap_or("");
        let detail_text_2 = datetime.to_string();
        let detail_2_width: f32 = calculate_text_width(&detail_text_2, &self.font, &font_scale);

        let detail_area_max_width = std::cmp::max(detail_1_width as u32, detail_2_width as u32);
        // draw detail 1
        info!("plotting detail text 1...");
        add_text(
            image,
            width + (standard_padding as f32 * 0.5) as u32 - detail_area_max_width as u32,
            height + (standard_padding as f32 * 1.5) as u32,
            &detail_text_1,
            &font_scale,
            &self.font,
            &BLACK,
        );
        // draw detail 2
        info!("plotting detail text 2...");
        add_text(
            image,
            width + (standard_padding as f32 * 0.5) as u32 - detail_area_max_width as u32,
            height + (standard_padding as f32 * 1.5) as u32 + font_height as u32,
            detail_text_2.as_str(),
            &font_scale,
            &self.font,
            &GRAY,
        );

        // add delimiter
        info!("plotting delimiter...");
        add_vertical_line(
            image,
            width - detail_area_max_width as u32,
            height + (standard_padding as f32 * 1.5 + font_height * 0.125) as u32,
            (font_height * 1.75) as u32,
            font_height as u32 / 32,
            &BLACK,
            &WHITE,
        );

        // add camera maker logo at bottom:right
        let logo = self
            .cache
            .get(exif_info.camera_maker.as_deref().unwrap_or(""));
        if logo.is_none() {
            return Ok(());
        }
        let logo = logo.unwrap();
        let (logo_origin_width, logo_origin_height) = logo.dimensions();
        let mut logo_new_height = font_height * 0.75;
        if (logo_origin_width as f32 / logo_origin_height as f32) <= 1.5 {
            // logo not too wide
            logo_new_height = font_height * 1.5;
        }
        let logo_new_width = logo_new_height * logo_origin_width as f32 / logo_origin_height as f32;
        let logo: ImageBuffer<Rgb<u8>, Vec<u8>> = resize(
            logo,
            logo_new_width as u32,
            logo_new_height as u32,
            FilterType::Lanczos3,
        );
        info!("plotting logo...");
        image.copy_from(
            &logo,
            width
                - detail_area_max_width as u32
                - logo_new_width as u32
                - (standard_padding as f32 * 0.5) as u32,
            height
                + (standard_padding as f32 * 1.5) as u32
                + ((font_height * 2.0 - logo_new_height) / 2.0) as u32,
        )?;

        Ok(())
    }
}
