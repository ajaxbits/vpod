use axum::{response::IntoResponse, routing::get, Router};
use std::{net::SocketAddr, str::FromStr};

mod audio;
mod feed;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let app = Router::new()
        .route("/:path_type", get(feed::serve_feed))
        .route("/:path_type/*val", get(feed::serve_feed))
        .route("/ep/:feed_id/:ep_id", get(audio::return_audio));

    let addr = SocketAddr::from_str("[::]:8888")?;
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handle_error(_err: std::io::Error) -> impl IntoResponse {
    (
        hyper::StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong...",
    )
}
