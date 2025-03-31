use std::{fs::File, io::Read};

pub fn read_font_data(filename: &str) -> Result<ab_glyph::FontVec, Box<dyn std::error::Error>> {
    let mut file = File::open(filename)?;
    let mut font_data = Vec::new();
    file.read_to_end(&mut font_data)?;
    Ok(ab_glyph::FontVec::try_from_vec(font_data)?)
}

pub fn read_sub_font_data(filename: &str) -> Option<ab_glyph::FontVec> {
    let mut file = File::open(filename).ok()?;
    let mut font_data = Vec::new();
    file.read_to_end(&mut font_data).ok()?;
    Some(ab_glyph::FontVec::try_from_vec(font_data).ok()?)
}
