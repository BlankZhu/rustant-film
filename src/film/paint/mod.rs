pub mod blank;
pub mod constant;
pub mod duel;
pub mod triangular;

use std::error::Error;

use ab_glyph::{Font, FontVec, PxScale, ScaleFont};
use constant::{BLANK_PAINTER, TRIANGLULAR_PAINTER};
use image::{GenericImage, ImageBuffer, Rgb, RgbImage};
use imageproc::drawing::draw_text_mut;
use triangular::TriangularPainter;
use blank::BlankPainter;
use duel::DuelPainter;

use crate::{
    entity::{ExifInfo, Padding, Position},
    LogoCache,
};

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
    padding: &Padding,
    color: &Rgb<u8>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = image.dimensions();
    let new_width = width + padding.left + padding.right;
    let new_height = height + padding.top + padding.bottom;

    let mut new_image: RgbImage = ImageBuffer::new(new_width, new_height);
    for x in 0..new_width {
        for y in 0..new_height {
            new_image.put_pixel(x, y, *color);
        }
    }

    new_image.copy_from(image, padding.left, padding.top)?;
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
fn get_text_scaled_length(text: &str, font: &impl Font, scale: &PxScale) -> u32 {
    let scaled_font = font.as_scaled(*scale);
    text.chars()
        .map(|c| scaled_font.h_advance(font.glyph_id(c)))
        .sum::<f32>() as u32
}

pub fn create_painter(
    painter_type: Option<String>,
    font: FontVec,
    cache: LogoCache,
    main_position: Option<Position>,
    pad_around: bool,
) -> Box<dyn Painter> {
    match painter_type {
        Some(pt) => match pt.to_lowercase().as_str() {
            TRIANGLULAR_PAINTER => Box::new(TriangularPainter::new(
                cache,
                font,
                main_position.unwrap_or(Position::BOTTOM),
                pad_around,
            )),
            BLANK_PAINTER => Box::new(BlankPainter::new(
                pad_around,
            )),
            _ => Box::new(TriangularPainter::new_normal(cache, font)),
        },
        None => Box::new(TriangularPainter::new_normal(cache, font)),
    }
}

pub fn create_canvas(width: u32, height: u32, color: Rgb<u8>) -> RgbImage {
    let mut canvas = RgbImage::new(width, height);
    for y in 0..height {
        for x in 0..width {
            canvas.put_pixel(x, y, color);
        }
    }
    canvas
}