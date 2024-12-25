#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExifInfo {
    pub artist: Option<String>,
    pub lens_model: Option<String>,
    pub camera_maker: Option<String>,
    pub camera_model: Option<String>,
    pub aperture: Option<String>,
    pub focal_length: Option<String>,
    pub exposure_time: Option<String>,
    pub iso: Option<String>,
    pub datetime: Option<String>,
}

impl ExifInfo {
    pub fn new_none() -> ExifInfo {
        ExifInfo {
            artist: None,
            lens_model: None,
            camera_maker: None,
            camera_model: None,
            aperture: None,
            focal_length: None,
            exposure_time: None,
            iso: None,
            datetime: None,
        }
    }

    pub fn new(exif: &exif::Exif) -> ExifInfo {
        let mut artist: Option<String> = None;
        let mut lens_model: Option<String> = None;
        let mut camera_maker: Option<String> = None;
        let mut camera_model: Option<String> = None;
        let mut aperture: Option<String> = None;
        let mut focal_length: Option<String> = None;
        let mut exposure_time: Option<String> = None;
        let mut iso: Option<String> = None;
        let mut datetime: Option<String> = None;

        if let Some(field) = exif.get_field(exif::Tag::Artist, exif::In::PRIMARY) {
            artist = Some(field.display_value().to_string().trim_matches('"').to_string());
        }
        if let Some(field) = exif.get_field(exif::Tag::LensModel, exif::In::PRIMARY) {
            lens_model = Some(field.display_value().to_string().trim_matches('"').to_string());
        }
        if let Some(field) = exif.get_field(exif::Tag::Make, exif::In::PRIMARY) {
            camera_maker = Some(field.display_value().to_string().trim_matches('"').to_string());
        }
        if let Some(field) = exif.get_field(exif::Tag::Model, exif::In::PRIMARY) {
            camera_model = Some(field.display_value().to_string().trim_matches('"').to_string());
        }
        if let Some(field) = exif.get_field(exif::Tag::FNumber, exif::In::PRIMARY) {
            aperture = Some(field.display_value().with_unit(exif).to_string());
        }
        if let Some(field) = exif.get_field(exif::Tag::FocalLength, exif::In::PRIMARY) {
            focal_length = Some(field.display_value().with_unit(exif).to_string());
        }
        if let Some(field) = exif.get_field(exif::Tag::ExposureTime, exif::In::PRIMARY) {
            exposure_time = Some(field.display_value().with_unit(exif).to_string());
        }
        if let Some(field) = exif.get_field(exif::Tag::PhotographicSensitivity, exif::In::PRIMARY) {
            iso = Some(field.display_value().with_unit(exif).to_string());
        }
        if let Some(field) = exif.get_field(exif::Tag::DateTimeOriginal, exif::In::PRIMARY) {
            datetime = Some(field.display_value().with_unit(exif).to_string());
        }

        ExifInfo {
            artist,
            lens_model,
            camera_maker,
            camera_model,
            aperture,
            focal_length,
            exposure_time,
            iso,
            datetime
        }
    }
}
