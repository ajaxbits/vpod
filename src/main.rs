use axum::{
    body::Body,
    extract::{Path, RequestParts},
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use core::panic;
// use gen_feed::gen_feed;
use std::{fs::File, io, net::SocketAddr, path::PathBuf, str::FromStr};
use tower::ServiceExt;
use ytd_rs::Arg;

mod episode;
mod feed;
mod gen_feed;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:cpfx/:channel_name", get(serve_rss))
        .route("/ep/:id", get(return_audio));

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

async fn serve_rss(Path((cpfx, id)): Path<(String, String)>) -> impl IntoResponse {
    let path = format!("{id}.xml");
    let feed = feed::Feed::new(&cpfx, &id);

    // let episodes: Vec<Episode> = get_recent_videos(channel_name)
    //     .into_iter()
    //     .map(Episode::from)
    //     .collect();
    // let feed = Feed::add_episodes(feed, episodes);

    let channel = rss::Channel::from(feed);
    let file = File::create(&path).unwrap_or_else(|_| panic!("could ot create {id}.xml"));
    channel.write_to(file).unwrap();

    let req = Request::builder().body(axum::body::Body::empty()).unwrap();

    let service =
        get_service(tower_http::services::ServeFile::new(path)).handle_error(handle_error);

    let result = service.oneshot(req).await;
    result
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
