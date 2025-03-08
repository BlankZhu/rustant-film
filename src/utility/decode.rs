use image::{
    codecs::{
        bmp::BmpDecoder, dds::DdsDecoder, farbfeld::FarbfeldDecoder, gif::GifDecoder,
        hdr::HdrDecoder, ico::IcoDecoder, jpeg::JpegDecoder, openexr::OpenExrDecoder,
        png::PngDecoder, pnm::PnmDecoder, qoi::QoiDecoder, tga::TgaDecoder, tiff::TiffDecoder,
        webp::WebPDecoder,
    },
    error::{ImageFormatHint, UnsupportedError, UnsupportedErrorKind},
    ImageDecoder, ImageError,
    ImageFormat::*,
};
use std::{fs, io::Cursor, path::PathBuf};

pub fn get_decoder(path: PathBuf) -> Result<Box<dyn ImageDecoder>, ImageError> {
    let bytes = fs::read(&path)?;
    let format = image::guess_format(&bytes)?;
    let cursor = Cursor::new(bytes);

    match format {
        Png => {
            let decoder = PngDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Jpeg => {
            let decoder = JpegDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Gif => {
            let decoder = GifDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        WebP => {
            let decoder = WebPDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Pnm => {
            let decoder = PnmDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Tiff => {
            let decoder = TiffDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Tga => {
            let decoder = TgaDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Dds => {
            let decoder = DdsDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Bmp => {
            let decoder = BmpDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Ico => {
            let decoder = IcoDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Hdr => {
            let decoder = HdrDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        OpenExr => {
            let decoder = OpenExrDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Farbfeld => {
            let decoder = FarbfeldDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        Qoi => {
            let decoder = QoiDecoder::new(cursor)?;
            Ok(Box::new(decoder))
        }
        _ => {
            let hint = ImageFormatHint::Exact(format);
            let kind = UnsupportedErrorKind::Format(hint.clone());
            let err = UnsupportedError::from_format_and_kind(hint, kind);
            Err(ImageError::Unsupported(err))
        }
    }
}
