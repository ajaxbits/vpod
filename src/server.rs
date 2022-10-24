use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Router};

pub async fn server() {
    let app = Router::new().route("/:name/:ep", get(get_episode));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("could not start server");
}

async fn get_episode() -> impl IntoResponse {
    todo!()
}
