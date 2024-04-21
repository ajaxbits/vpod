use axum::{response::IntoResponse, routing::get, Router};
use std::net::SocketAddr;

mod audio;
mod cli;
mod feed;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    let app = Router::new()
        .route("/:path_type", get(feed::serve_feed))
        .route("/:path_type/*val", get(feed::serve_feed))
        .route("/ep/:feed_id/:ep_id", get(audio::return_audio));

    let addr = SocketAddr::new(cli.host, cli.port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn handle_error(err: std::io::Error) -> impl IntoResponse {
    println!("{:#?}", err);
    (
        hyper::StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong...",
    )
}
