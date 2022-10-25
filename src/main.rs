use std::{net::SocketAddr, str::FromStr};

use axum::{
    extract::Path,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::get,
    Router,
};
use tokio::process::Command;

mod gen_feed;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:id", get(stream_audio))
        .route("/", get(serve_rss));

    let addr = SocketAddr::from_str("127.0.0.1:3000").expect("could not parse socketaddr");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("could not start server");
}

async fn stream_audio(Path(id): Path<String>) -> impl IntoResponse {
    let url = format!("https://www.youtube.com/watch?v={id}");
    let ytdl = Command::new("yt-dlp")
        .arg("--quiet")
        .arg("--format")
        .arg("bestaudio[protocol^=http][abr<100][ext=m4a]")
        .arg("-o")
        .arg("-")
        .arg(url)
        .output()
        .await;

    let body = ytdl.unwrap().stdout;

    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "audio/m4a".parse().unwrap());

    (StatusCode::OK, headers, body)
}

async fn serve_rss() -> impl IntoResponse {
    "todo!"
}
