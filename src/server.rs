use std::io::{BufReader, Cursor};

use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Query, State},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use exif::Reader;
use image::ImageEncoder;
use log::{debug, error, info, warn};

use crate::{
    api::state::{build_app_state, RustantFilmAppState},
    argument::Arguments,
    entity::{position, DevelopParams, ExifInfo},
    film::paint::create_painter,
};

async fn not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        "You're looking at a film that rustant-film cannot develop properly.",
    )
}

#[axum::debug_handler]
async fn develop(
    State(state): State<RustantFilmAppState>,
    Query(params): Query<DevelopParams>,
    mut mp: Multipart,
) -> Response {
    info!("got request with params: {:?}", params);

    while let Some(field) = mp.next_field().await.unwrap_or(None) {
        let name = field.name().unwrap_or_default().to_string();
        if name != "image" {
            warn!("skip useless field: {}", name);
            continue;
        }

        // read upload file into memory
        let data = match field.bytes().await {
            Ok(d) => d,
            Err(e) => {
                error!(
                    "failed to accept upload file, cause: {}, {}, {}",
                    e,
                    e.body_text(),
                    e.status()
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "cannt accept upload file",
                )
                    .into_response();
            }
        };

        // create painter
        let painter = params.painter;
        let position = position::from_str(params.pos.unwrap_or("".to_string()).as_str());
        let padding = params.pad.unwrap_or(false);
        let painter = create_painter(
            painter,
            state.font,
            state.sub_font,
            state.logos,
            position,
            padding,
        );

        // load exif info
        let cursor = Cursor::new(&data);    // exif::reader::Reader use only the first 4096 bytes, just to lower the memory cost
        let mut reader = BufReader::new(cursor);
        let exif = match Reader::new().read_from_container(&mut reader) {
            Ok(exif) => exif,
            Err(err) => {
                error!("cannot parse EXIF, cause: {}", err);
                return (StatusCode::BAD_REQUEST, "cannot parse EXIF from file").into_response();
            }
        };
        let exif_info = ExifInfo::new(&exif);
        debug!("exif info: {:?}", exif_info);

        // load input image - which can be asyn
        let image = match image::load_from_memory(&data) {
            Ok(image) => image,
            Err(err) => {
                error!("cannot read file as image cause: {}", err);
                return (StatusCode::BAD_REQUEST, "cannot read upload file as iamge")
                    .into_response();
            }
        };
        let mut image = image.to_rgb8();

        // draw the image
        if let Err(e) = painter.paint(&mut image, &exif_info) {
            error!("cannot paint image cause: {}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "cannt paint new image").into_response();
        }

        let mut buffer = Vec::new();
        if let Err(err) = image::codecs::jpeg::JpegEncoder::new(&mut buffer).write_image(
            &image,
            image.width(),
            image.height(),
            image::ColorType::Rgb8.into(),
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
        "expted file upload with field name 'image'",
    )
        .into_response()
}

pub async fn run(args: Arguments) -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    // tracing_subscriber::fmt::init();

    // get app state
    let state = build_app_state(args.logos, args.font, args.sub_font)?;

    // build app
    let app = Router::new()
        .route("/api/v1/develop", post(develop))
        .layer(DefaultBodyLimit::max(1024 * 1024 * 200))
        .fallback(not_found)
        .with_state(state);

    // listen
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
