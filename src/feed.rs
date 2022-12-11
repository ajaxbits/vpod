use futures::{Future, FutureExt, Stream, StreamExt};
use hyper::{body, server::conn::Http, Uri};
use rss::{
    extension::itunes::ITunesChannelExtensionBuilder, Channel, ChannelBuilder, ImageBuilder, Item,
};
use std::{
    collections::{BTreeMap, HashMap},
    io::BufReader,
};
use tokio::process::Command;
use youtube_dl::YoutubeDl;

use vpod::{get_channel_description, get_channel_image, yt_xml::YtFeed};

use super::episode::Episode;
use hyper_tls::HttpsConnector;

#[derive(Default, Debug, Clone, derive_builder::Builder)]
pub struct Feed {
    pub image: String, //url
    pub title: String,
    pub author: String,
    pub description: String,
    pub link: String,
    pub episodes: Option<Vec<Episode>>,
}

async fn add_episode_length(eps: Vec<Episode>) -> Vec<Episode> {
    let https = HttpsConnector::new();
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
                    let duration = text
                        .parse::<u32>()
                        .expect("could not parse duration as u32!");
                    duration
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

pub async fn update_feed(new_feed: Feed, old_feed: Feed) -> Feed {
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
    async fn get_ep_future(url: &str) {
        let process = Command::new("yt-dlp").args(vec!["--dump-json", "--quiet", url]);
        todo!()
    }
    pub async fn new(id: &str) -> Self {
        let feed = yt_feed_xml::Channel::new(id).await;
        let feed = Feed::from_yt_feed(feed).await;
        feed
        // let episodes = &mut feed.episodes.as_mut().unwrap();

        // let commands = episodes.into_iter().map(|ep| ep.get_yt_link()).map(|url| {
        //     Command::new("yt-dlp")
        //         .args(vec!["--dump-json", "--quiet", &url])
        //         .spawn()
        //         .expect("failed to spawn command")
        //         .wait_with_output()
        // });

        // let wow = futures::stream::iter(commands)
        //     .buffered(15)
        //     .map(|res| res.unwrap().stdout)
        //     .map(|bytes| String::from_utf8(bytes).unwrap())
        //     .collect::<Vec<_>>();

        // todo!()
    }

    pub fn add_episodes(self, episodes: Vec<Episode>) -> Self {
        Feed {
            episodes: Some(episodes),
            ..self
        }
    }

    pub async fn from_yt_feed(channel: yt_feed_xml::Channel) -> Self {
        let channel_image = get_channel_image(&channel.channel_url).await.unwrap();
        let channel_description = get_channel_description(&channel.channel_url).await.unwrap();

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

        FeedBuilder::default()
            .image(channel_image)
            .title(channel.title)
            .author(channel.author)
            .description(channel_description)
            .link(channel.channel_url)
            .episodes(Some(episodes))
            .build()
            .expect("could not build feed!")
    }
}

impl From<Feed> for Channel {
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

impl From<Channel> for Feed {
    fn from(channel: Channel) -> Self {
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
