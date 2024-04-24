use axum::{response::IntoResponse, routing::get, Router};
use std::io::IsTerminal;
use std::net::SocketAddr;
use std::process::ExitCode;
use tower_http::trace::TraceLayer;

mod cli;
mod error;
mod feed;
mod routes;
mod trace_layer;

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
    cli.instrumentation.setup()?;

    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(trace_layer::trace_layer_make_span_with)
        .on_request(trace_layer::trace_layer_on_request)
        .on_response(trace_layer::trace_layer_on_response);

    let app = Router::new()
        // .route("/", get(routes::get_home))
        .route("/:path_type", get(feed::serve_feed))
        .route("/:path_type/*val", get(feed::serve_feed))
        .layer(trace_layer);

    tracing::info!("Listening on {}:{}", cli.host, cli.port);
    let addr = SocketAddr::new(cli.host, cli.port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await?;

    Ok(ExitCode::SUCCESS)
}

async fn handle_error() -> (axum::http::StatusCode, &'static str) {
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong...",
    )
}
