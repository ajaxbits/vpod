use chrono::Duration;
use rss::{
    extension::itunes::ITunesChannelExtensionBuilder, Channel, ChannelBuilder, GuidBuilder,
    ImageBuilder, Item,
};
use std::{collections::BTreeMap, process::Command};

use crate::episode::gen_description;

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
        let url = format!("https://www.youtube.com/channel/{id}",);
        let resp = reqwest::get(url).await.unwrap().bytes().await.unwrap();

        let channel = Channel::read_from(&resp[..]).unwrap();
        let episodes: Vec<Episode> = channel.items().iter().collect();

        Feed {
            image: todo!(),
            title: todo!(),
            author: todo!(),
            description: todo!(),
            link,
            episodes: todo!(),
        }
    }

    pub fn add_episodes(self, episodes: Vec<Episode>) -> Self {
        Feed {
            episodes: Some(episodes),
            ..self
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
            image: itunes_metadata.image().unwrap().to_string(),
            author: itunes_metadata.author().unwrap().to_string(),
            description: channel.description().to_string(),
            link: channel.link().to_string(),
            episodes: Some(episodes),
        }
    }
}
