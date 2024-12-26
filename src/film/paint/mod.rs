pub mod bottom;
pub mod constant;
pub mod normal;
pub mod horizontal;
pub mod vertical;

use std::error::Error;

use ab_glyph::{Font, FontVec, PxScale, ScaleFont};
use constant::{BOTTOM_PAINTER, NORMAL_PAINTER};
use image::{GenericImage, ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;

use crate::entity::ExifInfo;

pub use bottom::BottomPainter;
pub use normal::NormalPainter;

use super::LogoCache;

pub trait Painter {
    fn paint(&self, image: &mut RgbImage, exif_info: &ExifInfo) -> Result<(), Box<dyn Error>>;
}

/// Add padding around the image.
///
/// This function will add padding with color to the given image.
///
/// # Examples
///
/// ```
/// add_padding(image, 10, 10, 10, 10, white);
/// ```
///
/// # Arguments
/// - `image`: a RgbImage to add padding
/// - `top`: top padding
/// - `bottom`: bottom padding
/// - `left`: left padding
/// - `right`: right padding
/// - `color`: color of the padding space
fn add_padding(
    image: &mut RgbImage,
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    color: &Rgb<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = image.dimensions();
    let new_width = width + left + right;
    let new_height = height + top + bottom;

    let mut new_image: RgbImage = ImageBuffer::new(new_width, new_height);
    for x in 0..new_width {
        for y in 0..new_height {
            new_image.put_pixel(x, y, *color);
        }
    }

    new_image.copy_from(image, left, top)?;
    *image = new_image;
    Ok(())
}

/// Add a text to image.
///
/// This function will add a text on (x,y) with font settings to the given image.
///
/// # Examples
///
/// ```
/// add_text(image, "hello", color, font_scale, &font, x y);
/// ```
///
/// #Arguments
/// - `image`: a RgbImage to add the text
/// - `x`: x-coordinate where the text starts
/// - `y`: y-coordinate where the text starts
/// - `text`: content of the text
/// - `scale`: font scale to control the text size
/// - `font`: font for the text
/// - `color`: color of the text
fn add_text(
    image: &mut RgbImage,
    x: u32,
    y: u32,
    text: &str,
    scale: &PxScale,
    font: &FontVec,
    color: &Rgb<u8>,
) {
    draw_text_mut(image, *color, x as i32, y as i32, *scale, &font, text);
}

/// Add a vertical line to image.
///
/// This function will draw a vertical line from (x,y) to (x,y+height) on a given image,
/// thickness and gradient color effects are supported.
///
/// # Examples
///
/// ```
/// add_vertical_line(image, x, y, 50, 10, black, gray);
/// ```
///
/// # Arguments
/// - `image`: a RgbImage to add the line
/// - `x`: x-coordinate where the line starts
/// - `y`: y-coordinate where the line starts
/// - `length`: length of the the line
/// - `thickness`: thickness of the line
/// - `start_color`: starting color of the gradient
/// - `end_color`: ending color of the gradient
fn add_vertical_line(
    image: &mut RgbImage,
    x: u32,
    y: u32,
    length: u32,
    thickness: u32,
    start_color: &Rgb<u8>,
    end_color: &Rgb<u8>,
) {
    let half_width = thickness / 2;
    for y in y..(y + length) {
        for x in x.saturating_sub(half_width)..=x + half_width {
            // Calculate the distance from the center of the line
            let distance = ((x as i32 - x as i32).abs()) as f32 / half_width as f32;

            // Interpolate between start_color and end_color based on the distance
            let r =
                (start_color[0] as f32 * (1.0 - distance) + end_color[0] as f32 * distance) as u8;
            let g =
                (start_color[1] as f32 * (1.0 - distance) + end_color[1] as f32 * distance) as u8;
            let b =
                (start_color[2] as f32 * (1.0 - distance) + end_color[2] as f32 * distance) as u8;

            // Set the pixel color at (x, y) to the calculated color
            if x < image.width() && y < image.height() {
                image.put_pixel(x, y, Rgb([r, g, b]));
            }
        }
    }
}

/// Calculate the pixel width of a text rendered.
///
/// # Examples
/// ```
/// let actual_width = calculate_text_width("hello", font, scale);
/// ```
///
/// # Arguments
/// - `text`: text content
/// - `font`: font to use
/// - `scale`: scale of the font
///
/// # Returns
/// - Returns the width of the final plotted text in pixel.
fn calculate_text_width(text: &str, font: &impl Font, scale: &PxScale) -> f32 {
    let scaled_font = font.as_scaled(*scale);
    text.chars()
        .map(|c| scaled_font.h_advance(font.glyph_id(c)))
        .sum()
}

pub fn create_painter(style: &str, cache: LogoCache, font: FontVec) -> Box<dyn Painter> {
    match style.to_ascii_lowercase().as_str() {
        NORMAL_PAINTER => Box::new(NormalPainter::new(cache, font)),
        BOTTOM_PAINTER => Box::new(BottomPainter::new(cache, font)),
        _ => Box::new(NormalPainter::new(cache, font)),
    }
}
