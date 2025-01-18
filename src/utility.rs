use std::{fs::File, io::Read};

use ab_glyph::FontVec;

pub fn read_font_data(filename: &str) -> Result<FontVec, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut font_data = Vec::new();
    file.read_to_end(&mut font_data)?;
    Ok(FontVec::try_from_vec(font_data)?)
}

pub fn read_sub_font_data(filename: &str) -> Option<FontVec> {
    let mut file = File::open(filename).ok()?;
    let mut font_data = Vec::new();
    file.read_to_end(&mut font_data).ok()?;
    Some(FontVec::try_from_vec(font_data).ok()?)
}
