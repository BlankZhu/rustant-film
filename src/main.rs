pub mod argument;
pub mod config;
pub mod info;
pub mod layout;

use clap::Parser;
use exif::Reader;
use log::{info, error};
use image::ImageReader;
use info::ExifInfo;
use std::{fs::File, io::BufReader, env};

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
    println!("exif info: {:?}", exif_info);

    // load logos from given directory

    work(args.input.as_str(), args.output.as_str()).unwrap();
}

fn work(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // load the exif info
    let file = File::open(input_path)?;
    let exif = Reader::new().read_from_container(&mut BufReader::new(&file))?;
    let exif_info = ExifInfo::new(&exif);

    // load the original image
    let origin_image = ImageReader::open(input_path)?.decode()?;
    let mut new_image = origin_image.to_rgb8();

    layout::add_layout_alpha(&mut new_image, &exif_info)?;
    new_image.save(output_path)?;
    Ok(())
}
