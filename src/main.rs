use axum::{
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use gen_feed::gen_feed;
use std::{io, net::SocketAddr, path::PathBuf, str::FromStr};
use tower::ServiceExt;
use ytd_rs::Arg;

mod channel_fetcher;
mod gen_feed;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:id", get(return_audio))
        .route("/", get(serve_rss));

    let addr = SocketAddr::from_str("127.0.0.1:3000").expect("could not parse socketaddr");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("could not start server");
}

async fn return_audio(Path(id): Path<String>) -> impl IntoResponse {
    let url = format!("https://www.youtube.com/watch?v={id}");
    let args = vec![
        Arg::new("--quiet"),
        Arg::new_with_arg("--format", "bestaudio[protocol^=http][abr<100][ext=m4a]"),
        Arg::new("--embed-metadata"),
        Arg::new("--embed-thumbnail"),
        Arg::new_with_arg("--output", "%(id)s.m4a"),
    ];
    let _ytd = ytd_rs::YoutubeDL::new(&PathBuf::from("./."), args, &url)
        .unwrap()
        .download();
    let path = format!("./{id}.m4a");

    let req = Request::builder()
        .uri(id)
        .body(axum::body::Body::empty())
        .unwrap();

    let service =
        get_service(tower_http::services::ServeFile::new(path)).handle_error(handle_error);

    let result = service.oneshot(req).await;

    result
}

async fn serve_rss() -> impl IntoResponse {
    gen_feed("TODO".to_owned()).await;
    "todo!"
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
