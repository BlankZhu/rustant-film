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
use image::{codecs::jpeg::JpegEncoder, ColorType, ImageEncoder};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, info_span, Span};

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

        // load input image
        let image = match image::load_from_memory(&data) {
            Ok(image) => image,
            Err(err) => {
                error!("cannot read file as image cause: {}", err);
                return (StatusCode::BAD_REQUEST, "cannot read upload file as image")
                    .into_response();
            }
        };
        let mut image = image.to_rgb8();

        // draw the image
        if let Err(err) = painter.paint(&mut image, &exif_info) {
            error!("cannot paint image cause: {}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "cannt paint new image").into_response();
        }

        let mut buffer = Vec::new();
        if let Err(err) = JpegEncoder::new(&mut buffer).write_image(
            &image,
            image.width(),
            image.height(),
            ColorType::Rgb8.into(),
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
