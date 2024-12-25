pub mod argument;
pub mod config;
pub mod entity;
pub mod film;

use ab_glyph::FontVec;
use clap::Parser;
use exif::Reader;
use film::{plot::{BottomPlotter, NormalPlotter, Plotter}, LogoCache};
use image::ImageReader;
use log::{info, error};
use entity::ExifInfo;
use std::{env, fs::File, io::{BufReader, Read}};

fn set_default_log_level() {
    env::set_var("RUST_LOG", env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()));
    pretty_env_logger::env_logger::init();
}

fn main() {
    let args = argument::Arguments::parse();
    set_default_log_level();
    info!("using input arguments: {:?}", args);

    // load input file
    let file = match File::open(args.input.as_str()) {
        Ok(f) => f,
        Err(e) => {
            error!("cannot open input file at {}, cause: {}", args.input.as_str(), e);
            return;
        }
    };
    // load input image
    let image = match ImageReader::open(&args.input) {
        Ok(f) => f,
        Err(e) => {
            error!("cannot read file {}, cause: {}", args.input, e);
            return;
        }
    };
    let mut image = match image.decode() {
        Ok(i) => i.to_rgb8(),
        Err(e) => {
            error!("cannot decode input image {}, cause: {}", args.input, e);
            return;
        }
    };

    // parse raw EXIF infos
    let exif = match Reader::new().read_from_container(&mut BufReader::new(&file)) {
        Ok(exif) => exif,
        Err(e) => {
            error!("cannot read EXIF from file {}, cause: {}", args.input.as_str(), e);
            return;
        }
    };
    // parse main EXIF infos
    let exif_info = ExifInfo::new(&exif);
    info!("exif info: {:?}", exif_info);

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
    
    // create plotter & plot
    let plotter = NormalPlotter::new(logo_cache, font);
    // let plotter = BottomPlotter::new(logo_cache, font);
    if let Err(e) = plotter.plot(&mut image, &exif_info) {
        error!("cannot plot image, cause: {}", e);
        return;
    }

    // save image
    if let Err(e) = image.save(&args.output) {
        error!("cannot save image to {}, cause: {}", args.output, e);
    }
}
