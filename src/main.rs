use axum::{response::IntoResponse, routing::get, Router};
use feed::get_playlist_feed;
use std::{net::SocketAddr, str::FromStr};

mod audio;
mod feed;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let app = Router::new()
        .route("/:path_type/*val", get(feed::get_channel_feed))
        // https://www.youtube.com/playlist?list=PLOIA4n5j7KcYW0VPvn9z8Wyzc0DOJfAzn
        .route("/playlist", get(feed::get_playlist_feed))
        .route("/ep/:id", get(audio::return_audio));

    let addr = SocketAddr::from_str("[::]:8080")?;
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
