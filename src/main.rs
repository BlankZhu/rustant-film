pub mod argument;
pub mod config;
pub mod entity;
pub mod film;

use ab_glyph::FontVec;
use clap::Parser;
use exif::Reader;
use film::{paint::create_painter, LogoCache};
use image::ImageReader;
use log::{error, info, warn};
use entity::ExifInfo;
use std::{env, fs::{self, File}, io::{BufReader, Read}, path::Path};

fn set_default_log_level() {
    env::set_var("RUST_LOG", env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()));
    pretty_env_logger::env_logger::init();
}

fn main() {
    let args = argument::Arguments::parse();
    set_default_log_level();
    info!("using input arguments: {:?}", args);

    // load logos from given directory
    let mut logo_cache = LogoCache::new();
    if let Err(e) = logo_cache.load(&args.logos) {
        error!("cannot read logos from file {}, cause: {}", args.logos, e);
        return;
    }

    // create FontVec from font data
    let mut file = match File::open(&args.font) {
        Ok(f) => f,
        Err(e) => {
            error!("cannot load font from file: {}, cause: {}", args.font, e);
            return;
        }
    };
    let mut font_data = Vec::new();
    if let Err(e) = file.read_to_end(&mut font_data) {
        error!("cannot read font file data from {}, cause: {}", args.font, e);
        return;
    }
    let font = match FontVec::try_from_vec(font_data) {
        Ok(font_vec) => font_vec,
        Err(e) => {
            error!("invalid font from {}, cause: {}", args.font, e);
            return;
        }
    };
    
    // create painter
    let painter = create_painter(&args.painter, logo_cache, font);

    // check output directory
    let output_directory_path = Path::new(&args.output);
    if let Err(e) = fs::create_dir_all(output_directory_path) {
        error!("cannot create output directory, cause: {}", e);
        return;
    }
    // plot input files
    let entries = match fs::read_dir(&args.input) {
        Ok(entries) => entries,
        Err(e) => {
            error!("cannot list input files under {}, cause: {}", args.input, e);
            return;
        }
    };
    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                warn!("cannot get file, cause: {}, skipping...", e);
                continue;
            }
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        // open the input file
        let file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                warn!("cannot open input file at {}, cause: {}", path.display(), e);
                continue;
            }
        };

        // load exif info
        let exif = match Reader::new().read_from_container(&mut BufReader::new(&file)) {
            Ok(exif) => exif,
            Err(e) => {
                warn!("cannot read EXIF from file {}, cause: {}", path.display(), e);
                continue;
            }
        };
        let exif_info = ExifInfo::new(&exif);
        info!("exif info: {:?}", exif_info);

        // load input image
        let image = match ImageReader::open(&path) {
            Ok(f) => f,
            Err(e) => {
                warn!("cannot read file {}, cause: {}", path.display(), e);
                continue;
            }
        };
        let mut image = match image.decode() {
            Ok(i) => i.to_rgb8(),
            Err(e) => {
                warn!("cannot decode input image {}, cause: {}", path.display(), e);
                continue;
            }
        };

        // plot the image
        if let Err(e) = painter.paint(&mut image, &exif_info) {
            error!("cannot plot image {}, cause: {}", path.display(), e);
            continue;
        }

        // save image
        let stem = match path.file_stem() {
            Some(s) => s.to_string_lossy().to_string(),
            None => {
                error!("cannot get stem name from {}", path.display());
                continue;
            }
        };
        let output_filename = format!("{}/{}.jpg", args.output, stem);
        if let Err(e) = image.save(&output_filename) {
            error!("cannot save image to {}, cause: {}", output_filename, e);
        }
    }
}
