use std::{
    collections::{BTreeMap, HashMap},
    io::BufReader,
};

use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::get_service,
};
use futures::StreamExt;
use hyper::body;
use rss::{extension::itunes::ITunesChannelExtensionBuilder, ChannelBuilder, ImageBuilder, Item};
use serde::Deserialize;
use tower::ServiceExt;

mod episode;
mod utils;
use episode::Episode;

pub async fn serve_feed(
    Path(YtPath { path_type, val }): Path<YtPath>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let yt_url = match path_type.clone() {
        YtPathType::Handle(handle) => format!("https://www.youtube.com/{handle}"),
        YtPathType::Abbrev(type_string)
        | YtPathType::Full(type_string)
        | YtPathType::Video(type_string)
        | YtPathType::Playlist(type_string)
        | YtPathType::User(type_string) => format!(
            "https://www.youtube.com/{}/{}",
            type_string,
            val.expect("This path type must have a val")
        ),
    };

    match path_type {
        YtPathType::Playlist(_) => {
            let pl_id = query
                .get("list")
                .expect("playlists need to have an id in the list query string")
                .to_owned();
            gen_rss(&pl_id, FeedType::Playlist).await
        }
        _ => {
            let channel_id = utils::get_channel_id(&yt_url)
                .await
                .expect("could not get channel_id");

            gen_rss(&channel_id, FeedType::Channel).await
        }
    }
}

async fn gen_rss(feed_id: &str, feed_type: FeedType) -> impl IntoResponse {
    let path = format!("{feed_id}/{feed_type}-{feed_id}.xml");
    let path = std::path::Path::new(&path);

    let req = hyper::Request::builder()
        .body(axum::body::Body::empty())
        .unwrap();

    let service =
        get_service(tower_http::services::ServeFile::new(&path)).handle_error(crate::handle_error);

    let feed = match &path.exists() {
        true => {
            let new_feed = Feed::new(&feed_id, feed_type);
            let old_file = std::fs::File::open(&path).unwrap();
            let new_feed = new_feed.await;

            let old_feed: Feed = rss::Channel::read_from(BufReader::new(&old_file))
                .unwrap()
                .into();

            update_feed(new_feed, old_feed).await
        }
        false => Feed::new(&feed_id, feed_type).await,
    };

    let channel = rss::Channel::from(feed.clone());

    let prefix = path.parent().expect("could not parse parent path");
    if prefix.exists() == false {
        std::fs::create_dir_all(&prefix).expect("could not create directory for podcast...");
    }

    let file =
        std::fs::File::create(&path).unwrap_or_else(|_| panic!("could not create {feed_id}.xml"));
    channel.write_to(file).unwrap();

    let result = service.oneshot(req).await;

    result
}

#[derive(Deserialize)]
pub struct YtPath {
    path_type: YtPathType,
    val: Option<String>,
}

#[derive(Clone, Debug)]
enum YtPathType {
    Handle(String),
    Abbrev(String),
    Full(String),
    User(String),
    Playlist(String),
    Video(String),
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
        } else if &s == "watch" {
            Ok(Self::Video(s))
        } else if &s == "playlist" {
            Ok(Self::Playlist(s))
        } else {
            Err(serde::de::Error::custom(format!(
                "Invalid path type '{}'",
                &s
            )))
        }
    }
}

#[derive(Debug, Clone)]
struct Feed {
    image: String, //url
    title: String,
    author: String,
    description: String,
    link: String,
    episodes: Option<Vec<Episode>>,
}

async fn add_episode_length(eps: Vec<Episode>) -> Vec<Episode> {
    let https = hyper_tls::HttpsConnector::new();
    let client = hyper::Client::builder().build::<_, hyper::Body>(https);

    let uris = eps
        .clone()
        .into_iter()
        .map(|ep| ep.get_yt_link())
        .map(|s| s.parse::<hyper::http::Uri>().unwrap());

    let urls = futures::stream::iter(uris).map(|uri| client.get(uri));

    let urls = urls
        .buffered(15)
        .then(|res| async {
            let res = res.expect("Error making request: {}");
            body::to_bytes(res).await.expect("err reading body!")
        })
        .then(|body| async {
            tokio::time::sleep(tokio::time::Duration::from_millis(5)).await;
            let body = String::from_utf8(body.into_iter().collect()).unwrap();
            let length = body.find("lengthSeconds");
            match length {
                Some(i) => {
                    let text = &body[i + 16..];
                    let end = text.find('"').unwrap();
                    let text = &text[..end];
                    text.parse::<u32>()
                        .expect("could not parse duration as u32!")
                }
                None => 1800,
            }
        })
        .collect::<Vec<u32>>()
        .await;

    eps.into_iter()
        .zip(urls.into_iter())
        .map(|(episode, length)| episode.set_length(length))
        .collect()
}

async fn update_feed(new_feed: Feed, old_feed: Feed) -> Feed {
    let old_eps = old_feed.episodes.unwrap();
    let mut new_eps = new_feed.episodes.as_ref().unwrap().to_owned();

    let tail = old_eps.last().unwrap();

    let start_index = match new_eps
        .iter()
        .rev()
        .position(|ep| ep.id.value() == tail.id.value())
    {
        Some(i) => i + 1,
        // TODO: fix how these are ordered so we don't have to do this
        None => 0,
    };

    // TODO: what if the new feed is entirely new?? I don't think I've accounted for this
    let eps = if start_index == 1 {
        old_eps
    } else {
        let new_eps = add_episode_length(new_eps.drain(start_index..).collect()).await;
        old_eps
            .into_iter()
            .chain(new_eps.into_iter())
            .enumerate()
            .map(|(count, ep)| ep.set_ep_number(Some(count.try_into().unwrap())))
            .collect()
    };

    Feed {
        episodes: Some(eps),
        ..new_feed
    }
}

enum FeedType {
    Channel,
    Playlist,
}

impl std::fmt::Display for FeedType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            Self::Channel => "channel",
            Self::Playlist => "playlist",
        };
        write!(f, "{}", s)
    }
}

impl Feed {
    async fn new(id: &str, feed_type: FeedType) -> Self {
        match feed_type {
            FeedType::Channel => {
                let feed = yt_feed_xml::Channel::new(id).await;
                Feed::from_yt_channel(feed).await
            }
            FeedType::Playlist => {
                let feed = yt_feed_xml::Playlist::new(id).await;
                Feed::from_yt_playlist(feed).await
            }
        }
    }

    async fn from_yt_channel(channel: yt_feed_xml::Channel) -> Self {
        let channel_image = utils::get_feed_image(&channel.url).await.unwrap();
        let channel_description = utils::get_feed_description(&channel.url).await.unwrap();
        let channel_id = channel.id;

        let episodes: Vec<yt_feed_xml::Video> = channel
            .videos
            .expect("this channel should have at least one video");

        let episodes: Vec<Episode> = process_videos(episodes, &channel_id).await;

        Feed {
            image: channel_image,
            title: match std::env::var("ENV") {
                Ok(var) if var == "staging" => format!("[β] {}", channel.title),
                _ => channel.title,
            },
            author: channel.author,
            description: channel_description,
            link: channel.url,
            episodes: Some(episodes),
        }
    }

    async fn from_yt_playlist(pl: yt_feed_xml::Playlist) -> Self {
        let image = utils::get_feed_image(&pl.url).await.unwrap();
        let description = utils::get_feed_description(&pl.url).await.unwrap();
        let pl_id = pl.id;

        let episodes: Vec<yt_feed_xml::Video> = pl
            .videos
            .expect("this playlist should have at least one video");

        let episodes: Vec<Episode> = process_videos(episodes, &pl_id).await;

        Feed {
            image,
            title: match std::env::var("ENV") {
                Ok(var) if var == "staging" => format!("[β] {}", pl.title),
                _ => pl.title,
            },
            author: pl.author,
            description,
            link: pl.url,
            episodes: Some(episodes),
        }
    }
}

async fn process_videos(vids: Vec<yt_feed_xml::Video>, feed_id: &str) -> Vec<Episode> {
    let eps = vids
        .into_iter()
        .map(|v| Episode::from_xml_video(v, feed_id))
        .collect();

    let eps = add_episode_length(eps).await;

    eps.into_iter()
        .filter(|ep| ep.duration_secs > 65)
        .filter(|ep| !ep.title.to_ascii_lowercase().contains("#shorts"))
        .rev()
        .enumerate()
        .map(|(count, ep)| ep.set_ep_number(Some(count.try_into().unwrap())))
        .collect()
}

impl From<Feed> for rss::Channel {
    fn from(feed: Feed) -> Self {
        let itunes_ns: BTreeMap<String, String> = BTreeMap::from([
            (
                "itunes".to_owned(),
                "http://www.itunes.com/dtds/podcast-1.0.dtd".to_owned(),
            ),
            (
                "content".to_owned(),
                "http://purl.org/rss/1.0/modules/content/".to_owned(),
            ),
        ]);

        let itunes_metadata = ITunesChannelExtensionBuilder::default()
            .author(Some(feed.author))
            .build();

        let image = ImageBuilder::default().url(feed.image).build();
        let episodes: Vec<Item> = feed
            .episodes
            .unwrap()
            .into_iter()
            .map(|ep| -> Item { ep.into() })
            .collect();

        ChannelBuilder::default()
            .namespaces(itunes_ns)
            .image(Some(image))
            .title(feed.title)
            .link(feed.link)
            .description(feed.description)
            .itunes_ext(Some(itunes_metadata))
            .items(episodes)
            .build()
    }
}

impl From<rss::Channel> for Feed {
    fn from(channel: rss::Channel) -> Self {
        let itunes_metadata = channel.itunes_ext().unwrap();
        let episodes: Vec<Episode> = channel
            .clone()
            .into_items()
            .into_iter()
            .map(Episode::from)
            .collect();

        Feed {
            title: channel.title().to_string(),
            image: channel.image().unwrap().url().to_string(),
            author: itunes_metadata.author().unwrap().to_string(),
            description: channel.description().to_string(),
            link: channel.link().to_string(),
            episodes: Some(episodes),
        }
    }
}
