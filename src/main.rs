use axum::{
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use core::panic;
use episode::Episode;
use feed::Feed;
use rss::Channel;
use std::{
    fs::File,
    io::{self, BufReader},
    net::SocketAddr,
    path::PathBuf,
    str::FromStr,
};
use tower::ServiceExt;
use ytd_rs::Arg;

mod episode;
mod feed;
mod gen_feed;
mod polling;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/:cpfx/:channel_name", get(serve_rss))
        .route("/ep/:id", get(return_audio));

    let addr = SocketAddr::from_str("[::]:8080").expect("could not parse socketaddr");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("could not start server");
}

async fn return_audio(Path(id): Path<String>) -> impl IntoResponse {
    let url = format!("https://www.youtube.com/watch?v={id}");
    let path = format!("{id}.m4a");
    if let false = std::path::Path::new(&path).exists() {
        let args = vec![
            Arg::new("--quiet"),
            Arg::new_with_arg("--format", "bestaudio[protocol^=http][abr<100][ext=m4a]"),
            Arg::new("--embed-metadata"),
            Arg::new("--embed-thumbnail"),
            Arg::new_with_arg("--sponsorblock-mark", "sponsor,selfpromo"),
            Arg::new_with_arg("--output", "%(id)s.m4a"),
        ];
        let _ytd = ytd_rs::YoutubeDL::new(&PathBuf::from("./."), args, &url)
            .unwrap()
            .download();
    }

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
    let yt_url = format!("https://www.youtube.com/{cpfx}/{id}");
    let id = vpod::get_channel_id(&yt_url)
        .await
        .expect("could not get channel_id");

    let path = format!("{id}.xml");

    let req = Request::builder().body(axum::body::Body::empty()).unwrap();

    let service =
        get_service(tower_http::services::ServeFile::new(&path)).handle_error(handle_error);

    let feed = match std::path::Path::new(&path).exists() {
        true => {
            let new_feed = Feed::new(&id);
            let old_file = File::open(&path).unwrap();
            let new_feed = new_feed.await;

            let old_feed: Feed = Channel::read_from(BufReader::new(&old_file))
                .unwrap()
                .into();

            feed::update_feed(new_feed, old_feed).await
        }
        false => feed::Feed::new(&id).await,
    };

    let channel = rss::Channel::from(feed.clone());
    let file = File::create(&path).unwrap_or_else(|_| panic!("could ot create {id}.xml"));
    channel.write_to(file).unwrap();

    let result = service.oneshot(req).await;

    result
}

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
