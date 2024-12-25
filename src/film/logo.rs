use std::{collections, error, fs};

use image::{ImageReader, RgbImage};
use log::info;

#[derive(Debug, Clone, PartialEq)]
pub struct LogoCache {
    cache: collections::HashMap<String, RgbImage>,
}

impl LogoCache {
    pub fn new() -> Self {
        LogoCache { cache: collections::HashMap::new() }
    }

    pub fn load(&mut self, logos_path: &str) -> Result<(), Box<dyn error::Error>> {
        let entries = fs::read_dir(logos_path)?;
        for entry in entries {
            let path = entry?.path();
            if path.is_file() {
                if let Some(filename) = path.file_stem() {
                    let logo_name = filename.to_string_lossy().to_ascii_lowercase();
                    info!("logo name: {}", logo_name);
                    let logo_image = ImageReader::open(path)?.decode()?.to_rgb8();
                    self.cache.insert(logo_name, logo_image);
                }
            }
        }
        Ok(())
    }

    pub fn get(&self, logo_name: &str) -> Option<&RgbImage> {
        self.cache.get(&logo_name.to_ascii_lowercase())
    }
}