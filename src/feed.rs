use rss::{extension::itunes::ITunesChannelExtensionBuilder, Channel, ChannelBuilder};
use std::{
    collections::{BTreeMap, HashMap},
    process::Command,
};
use youtube_dl::Playlist;

use super::episode::Episode;

#[derive(Debug, Clone)]
pub struct Feed {
    id: String,
    title: String,
    author: String,
    description: String,
    link: String,
    episodes: Option<HashMap<String, Episode>>,
}

impl Feed {
    fn new_from_id(&self, channel_name: &str) -> Self {
        let link = format!("https://www.youtube.com/c/{channel_name}");
        let command = Command::new("which").arg("yt-dlp").output().unwrap();

        let ytdlp_path = std::str::from_utf8(&command.stdout)
            .expect("failed to parse the stdout of the yt-dlp command")
            .trim();

        let command = Command::new(ytdlp_path)
            .args(["--dump-single-json", "--flat-playlist"])
            .arg(link)
            .output()
            .expect("yt-dlp ran with errors");

        let command = std::str::from_utf8(&command.stdout)
            .expect("failed to parse stdout")
            .trim();

        let json: serde_json::Value = serde_json::from_str(command).unwrap();
        let json = json.as_object().unwrap();

        Feed {
            id: json["uploader_id"]
                .as_str()
                .map(|val| val.to_owned())
                .expect("could not parse json uploader_val"),
            title: json["channel"]
                .as_str()
                .map(|val| val.to_owned())
                .expect("could not parse json uploader_val"),
            author: json["uploader"]
                .as_str()
                .map(|val| val.to_owned())
                .expect("could not parse json uploader_val"),
            description: json["description"]
                .as_str()
                .map(|val| val.to_owned())
                .expect("could not parse json uploader_val"),
            link: json["webpage_url"]
                .as_str()
                .map(|val| val.to_owned())
                .expect("could not parse json uploader_val"),
            episodes: None,
        }
    }
}

const ITUNES_NAMESPACES: BTreeMap<String, String> = BTreeMap::from([
    (
        "itunes".to_owned(),
        "http://www.itunes.com/dtds/podcast-1.0.dtd".to_owned(),
    ),
    (
        "content".to_owned(),
        "http://purl.org/rss/1.0/modules/content/".to_owned(),
    ),
]);

impl From<Feed> for Channel {
    fn from(feed: Feed) -> Self {
        let itunes_metadata = ITunesChannelExtensionBuilder::default()
            .author(Some(feed.author))
            .build();

        ChannelBuilder::default()
            .namespaces(ITUNES_NAMESPACES)
            .title(feed.title)
            .link(feed.link)
            .description(feed.description)
            .itunes_ext(Some(itunes_metadata))
            .items(
                feed.episodes
                    .unwrap()
                    .values()
                    .map(|ep| Into::<rss::Item>::into(*ep))
                    .collect::<Vec<rss::Item>>(),
            )
            .build()
    }
}
