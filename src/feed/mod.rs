use std::{collections::BTreeMap, io::BufReader};

use axum::{extract::Path, response::IntoResponse, routing::get_service};
use futures::StreamExt;
use hyper::body;
use rss::{extension::itunes::ITunesChannelExtensionBuilder, ChannelBuilder, ImageBuilder, Item};
use serde::Deserialize;
use tower::ServiceExt;

mod episode;
mod utils;
use episode::Episode;

// #[axum::debug_handler]
pub async fn serve_rss(Path(YtPath { path_type, val }): Path<YtPath>) -> impl IntoResponse {
    let yt_url = match path_type {
        YtPathType::Handle(handle) => format!("https://www.youtube.com/{handle}"),
        YtPathType::Abbrev(type_string)
        | YtPathType::Full(type_string)
        | YtPathType::User(type_string) => format!(
            "https://www.youtube.com/{}/{}",
            type_string,
            val.expect("This path type must have a val")
        ),
    };

    let id = utils::get_channel_id(&yt_url)
        .await
        .expect("could not get channel_id");

    let path = format!("{id}.xml");

    let req = hyper::Request::builder()
        .body(axum::body::Body::empty())
        .unwrap();

    let service =
        get_service(tower_http::services::ServeFile::new(&path)).handle_error(crate::handle_error);

    let feed = match std::path::Path::new(&path).exists() {
        true => {
            let new_feed = Feed::new(&id);
            let old_file = std::fs::File::open(&path).unwrap();
            let new_feed = new_feed.await;

            let old_feed: Feed = rss::Channel::read_from(BufReader::new(&old_file))
                .unwrap()
                .into();

            update_feed(new_feed, old_feed).await
        }
        false => Feed::new(&id).await,
    };

    let channel = rss::Channel::from(feed.clone());
    let file = std::fs::File::create(&path).unwrap_or_else(|_| panic!("could ot create {id}.xml"));
    channel.write_to(file).unwrap();

    let result = service.oneshot(req).await;

    result
}

#[derive(Deserialize)]
pub struct YtPath {
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

impl Feed {
    async fn new(id: &str) -> Self {
        let feed = yt_feed_xml::Channel::new(id).await;
        let feed = Feed::from_yt_feed(feed).await;
        feed
    }

    async fn from_yt_feed(channel: yt_feed_xml::Channel) -> Self {
        let channel_image = utils::get_channel_image(&channel.channel_url)
            .await
            .unwrap();
        let channel_description = utils::get_channel_description(&channel.channel_url)
            .await
            .unwrap();

        let episodes: Vec<Episode> = channel
            .videos
            .expect("this channel should have at least one video")
            .into_iter()
            .filter_map(
                |video| match video.title.to_ascii_lowercase().contains("#shorts") {
                    true => None,
                    false => Some(Episode::from(video)),
                },
            )
            .rev()
            .enumerate()
            .map(|(count, ep)| ep.set_ep_number(Some(count.try_into().unwrap())))
            .collect();

        let episodes = add_episode_length(episodes).await;

        Feed {
            image: channel_image,
            title: match std::env::var("ENV") {
                Ok(var) if var == "staging" => format!("🟡 {}", channel.title).to_owned(),
                _ => channel.title,
            },
            author: channel.author,
            description: channel_description,
            link: channel.channel_url,
            episodes: Some(episodes),
        }
    }
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
