use std::{
    io::{BufReader, Cursor},
    time::Duration,
};

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, MatchedPath, Multipart, Query, State},
    http::{header, Request, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use exif::Reader;
use image::ImageEncoder;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, info_span, warn, Span};

use crate::{
    api::state::{build_app_state, RustantFilmAppState},
    argument::Arguments,
    entity::{position, DevelopParams, ExifInfo},
    film::paint::create_painter,
    utility::decode::get_decoder,
};

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "You're looking at a film that rustant-film cannot develop properly.",
    )
}

#[tracing::instrument(skip(state, mp))]
#[axum::debug_handler]
async fn develop(
    State(state): State<RustantFilmAppState>,
    Query(params): Query<DevelopParams>,
    mut mp: Multipart,
) -> Response {
    info!("handling develop request");

    while let Some(field) = mp.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or_default().to_string();
        if name != "image" {
            debug!(name = name, "skip useless field");
            continue;
        }

        // read upload file into memory
        let data = match field.bytes().await {
            Ok(d) => d,
            Err(err) => {
                error!(
                    err_text = err.body_text(),
                    err_status = err.status().as_u16(),
                    "failed to accept upload file: {}",
                    err
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "cannt accept upload file",
                )
                    .into_response();
            }
        };

        // create painter
        let painter = params.painter.clone();
        let position = position::from_str(params.pos.clone().unwrap_or("".to_string()).as_str());
        let padding = params.pad.unwrap_or(false);
        let painter = create_painter(
            painter,
            state.font.clone(),
            state.sub_font.clone(),
            state.logos.clone(),
            position,
            padding,
        );

        // load exif info
        let cursor = Cursor::new(&data);
        let mut reader = BufReader::new(cursor);
        let exif = match Reader::new().read_from_container(&mut reader) {
            Ok(exif) => exif,
            Err(err) => {
                error!("cannot parse EXIF, cause: {}", err);
                return (StatusCode::BAD_REQUEST, "cannot parse EXIF from file").into_response();
            }
        };
        let exif_info = ExifInfo::new(&exif);
        debug!(exif_info = ?exif_info, "get exif info from image");

        // create decoder to read image
        let mut decoder = match get_decoder(data) {
            Ok(d) => d,
            Err(e) => {
                warn!("cannot get decoder cause: {}", e);
                continue;
            }
        };

        // get the potential ICC
        let icc_profile = match decoder.icc_profile() {
            Ok(p) => p,
            Err(e) => {
                warn!("cannot get ICC profile cause: {}", e);
                continue;
            }
        };
        if icc_profile.is_none() {
            debug!("no embedding ICC profile");
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
        if let Err(e) = decoder.read_image_boxed(&mut buffer) {
            warn!("cannot read image, cause: {}", e);
            continue;
        }
        let mut image = match image::RgbImage::from_raw(width, height, buffer) {
            Some(img) => img,
            None => {
                warn!("cannot decode image");
                continue;
            }
        };

        // draw the image
        if let Err(err) = painter.paint(&mut image, &exif_info) {
            error!("cannot paint image cause: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "cannt paint new image").into_response();
        }

        // create jpeg encoder
        let mut buffer = Vec::new();
        let mut encoder = image::codecs::jpeg::JpegEncoder::new(&mut buffer);
        if let Some(profile) = icc_profile {
            if let Err(e) = encoder.set_icc_profile(profile) {
                warn!("cannot set ICC profile to output file which may lead to incorrect color, cause: {}", e);
            }
        }

        if let Err(err) = encoder.encode(
            &image.as_raw(),
            image.width(),
            image.height(),
            color_type.into(),
        ) {
            error!("cannot convert new image to bytes, cause: {}", err);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "cannot handle upload image",
            )
                .into_response();
        }

        let resp = Response::builder()
            .header(header::CONTENT_TYPE, "image/jpeg")
            .header(
                header::CONTENT_DISPOSITION,
                r#"attachment; filename="image.jpeg""#,
            )
            .status(StatusCode::OK)
            .body(Body::from(buffer));
        let resp = match resp {
            Ok(r) => r,
            Err(err) => {
                error!("cannot build response, cause: {}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "cannot create response")
                    .into_response();
            }
        };
        return resp;
    }

    (
        StatusCode::BAD_REQUEST,
        "expect file upload with field name 'image'",
    )
        .into_response()
}

pub async fn run(args: Arguments) -> Result<(), Box<dyn std::error::Error>> {
    // setup app state
    let state = build_app_state(args.logos, args.font, args.sub_font)?;

    // build app
    let app = Router::new()
        .route("/api/v1/develop", post(develop))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 200))    // 200MB upload image limit
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    // todo: maybe a trace id here
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path
                    )
                })
                .on_response(|response: &Response, duration: Duration, _span: &Span| {
                    info!(status = response.status().as_u16(), duration = ?duration, "response completed");
                }),
        )
        .fallback(not_found)
        .with_state(state);

    // listen
    let listener = TcpListener::bind(format!("0.0.0.0:{}", args.port)).await?;
    debug!(port = args.port, "listening port...");
    axum::serve(listener, app).await?;

    Ok(())
}
