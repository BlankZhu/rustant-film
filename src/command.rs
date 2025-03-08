use std::{
    fs::{self, File},
    io::{BufReader, Cursor, Read},
    path::Path,
    sync::Arc,
};

use exif::Reader;
use image::{codecs::jpeg::JpegEncoder, ImageEncoder, RgbImage};
use log::{error, info, warn};

use crate::{
    argument::Arguments,
    entity::{position, ExifInfo},
    film::{paint::create_painter, LogoCache},
    utility::{
        decode::get_decoder,
        font::{read_font_data, read_sub_font_data},
    },
};

pub fn run(args: Arguments) {
    // load logos from given directory
    let mut logo_cache = LogoCache::new();
    if let Err(e) = logo_cache.load(&args.logos) {
        error!("cannot read logos from file {}, cause: {}", args.logos, e);
        return;
    }
    let logo_cache = Arc::new(logo_cache);

    // load the main font
    let font = match read_font_data(&args.font) {
        Ok(f) => f,
        Err(e) => {
            error!("cannot load font from file: {}, cause: {}", args.font, e);
            return;
        }
    };
    let font = Arc::new(font);

    // load the sub font
    let sub_font = args.sub_font.map_or(None, |sf| read_sub_font_data(&sf));
    let sub_font = Arc::new(sub_font);

    // create painter
    let painter = create_painter(
        args.painter,
        font,
        sub_font,
        logo_cache,
        position::from_str(args.position.unwrap_or("".to_string()).as_str()),
        args.padding,
    );

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
        let mut file = match File::open(&path) {
            Ok(f) => f,
            Err(e) => {
                warn!("cannot open input file at {}, cause: {}", path.display(), e);
                continue;
            }
        };
        // read file into data
        let mut buffer = Vec::new();
        if let Err(e) = file.read_to_end(&mut buffer) {
            warn!("cannot read file from {}, cause: {}", path.display(), e);
            continue;
        }
        let data = bytes::Bytes::from(buffer);
        let cursor = Cursor::new(&data);
        let mut reader = BufReader::new(cursor);

        // load exif info
        let exif = match Reader::new().read_from_container(&mut reader) {
            Ok(exif) => exif,
            Err(e) => {
                error!(
                    "cannot read EXIF from file {}, cause: {}",
                    path.display(),
                    e
                );
                continue;
            }
        };
        let exif_info = ExifInfo::new(&exif);
        info!("exif info: {:?}", exif_info);

        // load input image
        let image = match image::load_from_memory(&data) {
            Ok(image) => image,
            Err(e) => {
                error!(
                    "cannot read file from {} as image, cause: {}",
                    path.display(),
                    e
                );
                continue;
            }
        };
        let mut image = image.to_rgb8();

        // paint the image
        if let Err(e) = painter.paint(&mut image, &exif_info) {
            error!("cannot paint image {}, cause: {}", path.display(), e);
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

// todo: replace the above function with this in the future
pub fn run2(args: Arguments) {
    // load logos from given directory
    let mut logo_cache = LogoCache::new();
    if let Err(e) = logo_cache.load(&args.logos) {
        error!("cannot read logos from file {}, cause: {}", args.logos, e);
        return;
    }
    let logo_cache = Arc::new(logo_cache);

    // load the main font
    let font = match read_font_data(&args.font) {
        Ok(f) => f,
        Err(e) => {
            error!("cannot load font from file: {}, cause: {}", args.font, e);
            return;
        }
    };
    let font = Arc::new(font);

    // load the sub font
    let sub_font = args.sub_font.map_or(None, |sf| read_sub_font_data(&sf));
    let sub_font = Arc::new(sub_font);

    // create painter
    let painter = create_painter(
        args.painter,
        font,
        sub_font,
        logo_cache,
        position::from_str(args.position.unwrap_or("".to_string()).as_str()),
        args.padding,
    );

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

        // read the file into bytes
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(e) => {
                warn!("cannot read file from {}, cause: {}", path.display(), e);
                continue;
            }
        };

        // load EXIF info
        let cursor = Cursor::new(&bytes);
        let mut reader = BufReader::new(cursor);
        let exif = match Reader::new().read_from_container(&mut reader) {
            Ok(exif) => exif,
            Err(e) => {
                error!(
                    "cannot read EXIF from file {}, cause: {}",
                    path.display(),
                    e
                );
                continue;
            }
        };
        let exif_info = ExifInfo::new(&exif);
        info!("exif info: {:?}", exif_info);

        // create decoder to read image
        let mut decoder = match get_decoder(bytes) {
            Ok(d) => d,
            Err(e) => {
                warn!(
                    "cannot get decoder from file {}, cause: {}",
                    path.display(),
                    e
                );
                continue;
            }
        };

        // get the potential ICC
        let icc_profile = match decoder.icc_profile() {
            Ok(p) => p,
            Err(e) => {
                warn!(
                    "cannot get ICC profile from file {}, cause: {}",
                    path.display(),
                    e
                );
                continue;
            }
        };
        if icc_profile.is_none() {
            warn!("no embedding ICC profile in {}", path.display());
        }

        // decode the image into RgbImage
        let (width, height) = decoder.dimensions();
        let color_type = decoder.color_type();
        if color_type != image::ColorType::Rgb8 {
            warn!("cannot handle color type {:?}, skipping...", color_type);
            continue;
        }
        let total_bytes = decoder.total_bytes() as usize;
        let mut buffer = vec![0u8; total_bytes];
        match decoder.read_image_boxed(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                warn!(
                    "cannot read image from file {}, cause: {}",
                    path.display(),
                    e
                );
                continue;
            }
        }
        let mut image = match RgbImage::from_raw(width, height, buffer) {
            Some(img) => img,
            None => {
                warn!("cannot decode image from file {}", path.display());
                continue;
            }
        };

        // paint the image
        if let Err(e) = painter.paint(&mut image, &exif_info) {
            error!("cannot paint image {}, cause: {}", path.display(), e);
            continue;
        }

        // save the image with possible ICC profile
        let stem = match path.file_stem() {
            Some(s) => s.to_string_lossy().to_string(),
            None => {
                error!("cannot get stem name from {}", path.display());
                continue;
            }
        };
        let output_filename = format!("{}/{}.jpg", args.output, stem);
        let outupt_file = match File::create(&output_filename) {
            Ok(f) => f,
            Err(e) => {
                error!(
                    "cannot create output file at {}, cause: {}",
                    output_filename, e
                );
                continue;
            }
        };
        let mut encoder = JpegEncoder::new(&outupt_file);
        if let Some(profile) = icc_profile {
            if let Err(e) = encoder.set_icc_profile(profile) {
                warn!("cannot set ICC profile to output file which may lead to incorrect color, cause: {}", e);
            }
        }
        if let Err(e) = encoder.encode(
            &image.as_raw(),
            image.width(),
            image.height(),
            color_type.into(),
        ) {
            error!("cannot encode image to {}, cause: {}", output_filename, e);
            continue;
        }
    }
}
