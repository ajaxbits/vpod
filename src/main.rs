use axum::{
    extract::Path,
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::{get, get_service},
    Router,
};
use core::panic;
use feed::Feed;
use rss::Channel;
use serde::Deserialize;
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
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let app = Router::new()
        .route("/:path_type/*val", get(serve_rss))
        .route("/ep/:id", get(return_audio));

    let addr = SocketAddr::from_str("[::]:8080").expect("could not parse socketaddr");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("could not start server");

    Ok(())
}

#[derive(Deserialize)]
struct YtPath {
    path_type: YtPathType,
    val: Option<String>,
}

#[derive(Debug)]
enum YtPathType {
    Handle(String),
    Abbrev(String),
    Full(String),
    User(String),
}

impl<'de> Deserialize<'de> for YtPathType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if &s[..1] == "@" {
            Ok(Self::Handle(s))
        } else if &s == "c" {
            Ok(Self::Abbrev(s))
        } else if &s == "channel" {
            Ok(Self::Full(s))
        } else if &s == "user" {
            Ok(Self::User(s))
        } else {
            Err(serde::de::Error::custom(format!(
                "Invalid path type '{}'",
                &s
            )))
        }
    }
}

// #[axum::debug_handler]
async fn serve_rss(Path(YtPath { path_type, val }): Path<YtPath>) -> impl IntoResponse {
    let yt_url = match path_type {
        YtPathType::Handle(handle) => format!("https://www.youtube.com/{handle}"),
        YtPathType::Abbrev(type_string)
        | YtPathType::Full(type_string)
        | YtPathType::User(type_string) => format!(
            "https://www.youtube.com/{}/{}",
            type_string,
            val.expect(&format!("This path type must have a val"))
        ),
    };

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

// #[axum::debug_handler]
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

async fn handle_error(_err: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...")
}
