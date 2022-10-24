use super::{get_link, get_ytd_out, invoke_ytd};
use std::net::SocketAddr;

use axum::{extract::Path, response::IntoResponse, routing::get, Router};

async fn get_episode(Path((name, ep)): Path<(String, String)>) -> impl IntoResponse {
    get_link(get_ytd_out(invoke_ytd()))
}

pub async fn server() {
    let app = Router::new().route("/:name/:ep", get(get_episode));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("could not start server");
}
