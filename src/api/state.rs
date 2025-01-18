use std::sync::Arc;

use ab_glyph::FontVec;
use log::error;

use crate::{
    film::LogoCache,
    utility::{read_font_data, read_sub_font_data},
};

#[derive(Debug, Clone)]
pub struct RustantFilmAppState {
    pub logos: Arc<LogoCache>,
    pub font: Arc<FontVec>,
    pub sub_font: Arc<Option<FontVec>>,
}

pub fn build_app_state(
    logos_dir: String,
    font_filename: String,
    sub_font_filename: Option<String>,
) -> Result<RustantFilmAppState, Box<dyn std::error::Error>> {
    // load logos from given directory
    let mut logo_cache = LogoCache::new();
    if let Err(e) = logo_cache.load(&logos_dir) {
        error!("cannot read logos from file {}, cause: {}", logos_dir, e);
        return Err(e);
    }

    let font = match read_font_data(&font_filename) {
        Ok(f) => f,
        Err(e) => {
            error!(
                "cannot load font from file: {}, cause: {}",
                font_filename, e
            );
            return Err(e);
        }
    };

    let sub_font = sub_font_filename.map_or(None, |sf| read_sub_font_data(&sf));

    Ok(RustantFilmAppState {
        logos: Arc::new(logo_cache),
        font: Arc::new(font),
        sub_font: Arc::new(sub_font),
    })
}
