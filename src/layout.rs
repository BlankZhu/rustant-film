use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use image::{imageops::{resize, FilterType}, GenericImage, ImageBuffer, ImageReader, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;

use crate::info::ExifInfo;

fn add_padding(
    image: &mut RgbImage,
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = image.dimensions();
    let new_width = width + left + right;
    let new_height = height + top + bottom;

    let mut new_image: RgbImage = ImageBuffer::new(new_width, new_height);
    for x in 0..new_width {
        for y in 0..new_height {
            new_image.put_pixel(x, y, Rgb([255, 255, 255]));
        }
    }

    new_image.copy_from(image, left, top)?;
    *image = new_image;
    Ok(())
}

fn add_text(
    image: &mut RgbImage,
    text: &str,
    color: Rgb<u8>,
    scale: PxScale,
    font: &FontRef<'_>,
    x: u32,
    y: u32,
) {
    draw_text_mut(image, color, x as i32, y as i32, scale, font, text);
}

fn draw_gradient_vertical_line(
    image: &mut RgbImage, // RgbaImage for full alpha control
    x_pos: u32,           // x-coordinate where the line will be drawn
    y_pos: u32,           // y-coordinate where the line will be drawn
    line_height: u32,     // Height of the image
    line_width: u32,      // Width (thickness) of the line
    start_color: Rgb<u8>, // Starting color of the gradient
    end_color: Rgb<u8>,   // Ending color of the gradient
) {
    let half_width = line_width / 2;
    for y in y_pos..(y_pos + line_height) {
        for x in x_pos.saturating_sub(half_width)..=x_pos + half_width {
            // Calculate the distance from the center of the line
            let distance = ((x as i32 - x_pos as i32).abs()) as f32 / half_width as f32;

            // Interpolate between start_color and end_color based on the distance
            let r =
                (start_color[0] as f32 * (1.0 - distance) + end_color[0] as f32 * distance) as u8;
            let g =
                (start_color[1] as f32 * (1.0 - distance) + end_color[1] as f32 * distance) as u8;
            let b =
                (start_color[2] as f32 * (1.0 - distance) + end_color[2] as f32 * distance) as u8;

            // Set the pixel color at (x, y) to the calculated color
            if x >= 0 && x < image.width() && y < image.height() {
                image.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
    }
}

fn calculate_line_width(text: &str, font: &FontRef<'_>, scale: &PxScale) -> f32 {
    let scaled_font = font.as_scaled(*scale);
    text.chars()
        .map(|c| scaled_font.h_advance(font.glyph_id(c)))
        .sum()
}

fn load_logo(maker: &str) -> Result<Option<RgbImage>, Box<dyn std::error::Error>> {
    if maker.to_ascii_lowercase() == "sony" {
        let logo = ImageReader::open("./resources/logo/sony.jpg")?.decode()?;
        let logo = logo.to_rgb8();
        return Ok(Some(logo))
    }
    Ok(None)
}

pub fn add_layout_alpha(
    image: &mut RgbImage,
    exif: &ExifInfo,
) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = image.dimensions();
    let standard_padding_ratio = (1.0 / 1.618033988749) / 16.0;
    let bottom_padding = (height as f64 * standard_padding_ratio * 2.0) as u32;
    let other_padding = (height as f64 * standard_padding_ratio) as u32;

    // add padding
    add_padding(
        image,
        other_padding,
        bottom_padding,
        other_padding,
        other_padding,
    )?;

    // some common setting for text add-ons
    let black = Rgb([0, 0, 0]);
    let gray = Rgb([125, 127, 124]);
    let white = Rgb([255, 255, 255]);
    let font_width = other_padding as f32 * 0.5;
    let font_height = other_padding as f32 * 0.5;
    let font_scale = PxScale {
        x: font_width,
        y: font_height,
    };
    let font = FontRef::try_from_slice(include_bytes!("../resources/font/OpenSans-Medium.ttf"))?;

    // add lens text at bottom:left
    let lens_model_text = exif.lens_model.as_deref().unwrap_or("");
    add_text(
        image,
        lens_model_text,
        black,
        font_scale,
        &font,
        other_padding + (other_padding as f32 * 0.5) as u32,
        height + (other_padding as f32 * 1.5) as u32,
    );

    // add camera text at bottom:left
    let camera_model_text = exif.camera_model.as_deref().unwrap_or("");
    add_text(
        image,
        camera_model_text,
        gray,
        font_scale,
        &font,
        other_padding + (other_padding as f32 * 0.5) as u32,
        height + (other_padding as f32 * 1.5) as u32 + font_height as u32,
    );

    // setup detailed info 1 at bottom:right
    let focal_length: String = exif
        .focal_length
        .clone()
        .unwrap_or("".to_string())
        .chars()
        .filter(|&c| c != ' ')
        .collect();
    let apreture: String = exif
        .aperture
        .clone()
        .unwrap_or("".to_string())
        .chars()
        .filter(|&c| c != ' ')
        .collect();
    let exposure: String = exif
        .exposure_time
        .clone()
        .unwrap_or("".to_string())
        .chars()
        .filter(|&c| c != ' ')
        .collect();
    let iso: String = exif
        .iso
        .clone()
        .unwrap_or("".to_string())
        .chars()
        .filter(|&c| c != ' ')
        .collect();
    let details = vec![focal_length, apreture, exposure, iso];
    let detail_text_1 = details.join(" ");
    let detail_1_width: f32 = calculate_line_width(&detail_text_1, &font, &font_scale);

    // add detail info 2 at bottom:right
    let datetime = exif.datetime.as_deref().unwrap_or("");
    let detail_text_2 = datetime.to_string();
    let detail_2_width: f32 = calculate_line_width(&detail_text_2, &font, &font_scale);

    let detail_area_max_width = std::cmp::max(detail_1_width as u32, detail_2_width as u32);
    // draw detail 1
    add_text(
        image,
        &detail_text_1,
        black,
        font_scale,
        &font,
        width + (other_padding as f32 * 0.5) as u32 - detail_area_max_width as u32,
        height + (other_padding as f32 * 1.5) as u32,
    );
    // draw detail 2
    add_text(
        image,
        detail_text_2.as_str(),
        gray,
        font_scale,
        &font,
        width + (other_padding as f32 * 0.5) as u32 - detail_area_max_width as u32,
        height + (other_padding as f32 * 1.5) as u32 + font_height as u32,
    );
    // other_padding + (other_padding as f32 * 0.5) as u32,

    // add delimiter
    draw_gradient_vertical_line(
        image,
        width - detail_area_max_width as u32,
        height + (other_padding as f32 * 1.5 + font_height * 0.25) as u32,
        (font_height * 1.5) as u32,
        other_padding / 64,
        black,
        white,
    );

    // add camera maker logo at bottom:right
    let logo = load_logo(exif.camera_maker.as_deref().unwrap_or(""))?;
    if logo.is_none() {
        return Ok(());
    }
    let logo = logo.unwrap();
    let (logo_origin_width, logo_origin_height) = logo.dimensions();
    let logo_new_height = font_height * 0.75;
    let logo_new_width = logo_new_height * logo_origin_width as f32 / logo_origin_height as f32;
    let logo: ImageBuffer<Rgb<u8>, Vec<u8>> = resize(&logo, logo_new_width as u32, logo_new_height as u32, FilterType::Lanczos3);
    image.copy_from(
        &logo, 
        width - detail_area_max_width as u32 - logo_new_width as u32 - (other_padding as f32 * 0.5) as u32,
        height + (other_padding as f32 * 1.5) as u32 + ((font_height * 2.0 - logo_new_height) / 2.0) as u32)?;

    Ok(())
}
