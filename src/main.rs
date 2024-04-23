use axum::{response::IntoResponse, routing::get, Router};
use std::io::IsTerminal;
use std::net::SocketAddr;
use std::process::ExitCode;

mod audio;
mod cli;
mod error;
mod feed;

use crate::cli::Cli;
use crate::error::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<ExitCode> {
    color_eyre::config::HookBuilder::default()
        .theme(if !std::io::stderr().is_terminal() {
            color_eyre::config::Theme::new()
        } else {
            color_eyre::config::Theme::dark()
        })
        .install()?;

    let cli = Cli::parse();
    let trace_layer = tower_http::trace::TraceLayer::new_for_http().on_request(
        |_req: &hyper::Request<hyper::Body>, _span: &tracing::Span| tracing::trace!("got request"),
    );

    let app = Router::new()
        .route("/:path_type", get(feed::serve_feed))
        .route("/:path_type/*val", get(feed::serve_feed))
        .route("/ep/:feed_id/:ep_id", get(audio::return_audio))
        .layer(trace_layer);

    tracing::info!("Listening on {}:{}", cli.host, cli.port);
    let addr = SocketAddr::new(cli.host, cli.port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(ExitCode::SUCCESS)
}

async fn handle_error(err: std::io::Error) -> impl IntoResponse {
    println!("{:#?}", err);
    (
        hyper::StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong...",
    )
}
