use std::{
    fs::{self, File},
    io::{BufReader, Cursor, Read},
    path::{Path, PathBuf},
    sync::Arc,
};

use bytes::Bytes;
use exif::Reader;
use log::{error, info, warn};
use tokio::task;

use crate::{
    argument::Arguments,
    entity::{position, ExifInfo},
    film::{
        paint::{create_painter, Painter},
        LogoCache,
    },
    utility::{read_font_data, read_sub_font_data},
};

pub async fn run(args: Arguments) {
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
    let painter = Arc::new(painter);

    // check output directory
    let output_directory_path = Path::new(&args.output);
    if let Err(e) = fs::create_dir_all(output_directory_path) {
        error!("cannot create output directory, cause: {}", e);
        return;
    }
    // list input files
    let entries = match fs::read_dir(&args.input) {
        Ok(entries) => entries,
        Err(e) => {
            error!("cannot list input files under {}, cause: {}", args.input, e);
            return;
        }
    };

    let mut handles = Vec::new();
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

        let painter_clone = painter.clone();
        let handle = task::spawn(develop(path, painter_clone, args.output.clone()));
        handles.push(handle);
    }

    for handle in handles {
        if let Err(e) = handle.await {
            error!("failed to join future, cause: {}", e);
            continue;
        }
    }
}

async fn develop(path: PathBuf, painter: Arc<Box<dyn Painter>>, output: String) {
    // open the input file
    let mut file = match File::open(&path) {
        Ok(f) => f,
        Err(e) => {
            error!("cannot open file at {}, cause: {}", path.display(), e);
            return;
        }
    };

    // read file into bytes data
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        error!("cannot read file from {}, cause: {}", path.display(), e);
        return;
    }
    let data = Bytes::from(buffer);
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
            return;
        }
    };
    let exif_info = ExifInfo::new(&exif);
    info!("handling exif info: {}", exif_info); // fixme: implement a display trait for it

    // load input image
    let image = match image::load_from_memory(&data) {
        Ok(image) => image,
        Err(e) => {
            error!(
                "cannot read file from {} as image, cause: {}",
                path.display(),
                e
            );
            return;
        }
    };
    let mut image = image.to_rgb8();

    // paint the image
    if let Err(e) = painter.paint(&mut image, &exif_info) {
        error!("cannot paint image {}, cause: {}", path.display(), e);
        return;
    }

    // save the image
    let stem = match path.file_stem() {
        Some(s) => s.to_string_lossy().to_string(),
        None => {
            error!("cannot get stem name from {}", path.display());
            return;
        }
    };
    let output_filename = format!("{}/{}.jpg", output, stem);
    if let Err(e) = image.save(&output_filename) {
        error!("cannot save image to {}, cause: {}", output_filename, e);
    }
    info!("new image developed at {}", output_filename);
}
