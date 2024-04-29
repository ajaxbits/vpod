use axum::{routing::get, Router};
use std::io::IsTerminal;
use std::net::SocketAddr;
use std::process::ExitCode;
use tower_http::trace::TraceLayer;

mod audio;
mod cli;
mod error;
mod feed;
mod trace_layer;

use crate::cli::Cli;
use crate::error::Result;
use clap::Parser;

#[derive(Debug, Clone)]
struct AppState {
    client: reqwest::Client,
    episode_url: String,
}

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

    let state = AppState {
        client: reqwest::Client::new(),
        episode_url: cli.episode_url.to_string(),
    };

    let app = Router::new()
        .route("/:path_type", get(feed::serve_feed))
        .route("/ep/:feed_id/:file_name", get(audio::return_audio))
        .layer(trace_layer)
        .with_state(state);

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
