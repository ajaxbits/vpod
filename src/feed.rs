use futures::StreamExt;
use hyper::body;
use rss::{
    extension::itunes::ITunesChannelExtensionBuilder, Channel, ChannelBuilder, ImageBuilder, Item,
};
use std::collections::{BTreeMap, HashMap};

use vpod::{get_channel_description, get_channel_image, yt_xml::YtFeed};

use super::episode::Episode;

#[derive(Debug, Clone)]
pub struct Feed {
    pub image: String, //url
    pub title: String,
    pub author: String,
    pub description: String,
    pub link: String,
    pub episodes: Option<Vec<Episode>>,
}

impl Feed {
    pub async fn new(id: &str) -> Self {
        let feed = YtFeed::from_channel_id(id).await;
        let feed = Feed::from_yt_feed(feed).await;
        feed
    }

    pub fn add_episodes(self, episodes: Vec<Episode>) -> Self {
        Feed {
            episodes: Some(episodes),
            ..self
        }
    }

    pub async fn from_yt_feed(feed: YtFeed) -> Self {
        let channel_image = get_channel_image(&feed.author.uri.value).await.unwrap();
        let channel_description = get_channel_description(&feed.author.uri.value)
            .await
            .unwrap();

        let episodes: Vec<Episode> = feed
            .videos
            .into_iter()
            .filter_map(|video| match video.title.value.contains("#Shorts") {
                true => None,
                false => Some(Episode::from(video)),
            })
            .enumerate()
            .map(|(count, ep)| ep.set_ep_number(Some(count.try_into().unwrap())))
            .collect();

        let client = hyper::Client::new();

        let uris = episodes
            .clone()
            .into_iter()
            .map(|ep| ep.get_yt_link())
            .map(|s| s.parse::<hyper::http::Uri>().unwrap());

        let urls = futures::stream::iter(uris)
            .map(move |uri| client.get(uri))
            .buffered(15)
            .then(|res| async {
                let res = res.expect("Error making request: {}");
                body::to_bytes(res).await.expect("err reading body!")
            })
            .then(|body| async {
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

        let episodes = episodes
            .into_iter()
            .zip(urls.into_iter())
            .map(|(episode, length)| episode.set_length(length))
            .collect();

        Feed {
            image: channel_image,
            title: feed.title.value,
            author: feed.author.name.value,
            description: channel_description,
            link: feed.author.uri.value,
            episodes: Some(episodes),
        }
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
