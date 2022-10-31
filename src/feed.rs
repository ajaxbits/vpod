use rss::{
    extension::itunes::ITunesChannelExtensionBuilder, Channel, ChannelBuilder, ImageBuilder, Item,
};
use std::{
    collections::{BTreeMap, HashMap},
    process::Command,
};

use super::episode::Episode;

#[derive(Debug, Clone)]
pub struct Feed {
    image: String, //url
    title: String,
    author: String,
    description: String,
    link: String,
    episodes: Option<Vec<Episode>>,
}

impl Feed {
    pub fn new_from_name(channel_name: &str) -> Self {
        let link = format!("https://www.youtube.com/c/{channel_name}");
        let command = Command::new("which").arg("yt-dlp").output().unwrap();

        let ytdlp_path = std::str::from_utf8(&command.stdout)
            .expect("failed to parse the stdout of the yt-dlp command")
            .trim();

        println!("yt-dlp path is: {:#?}", ytdlp_path.clone());

        let mut command = Command::new(ytdlp_path);
        command
            .args(["--dump-single-json", "--flat-playlist", "--write-thumbnail"])
            .arg("--playlist-items")
            .arg("0")
            .arg(link);
        let command = command.output().expect("yt-dlp ran with errors");

        let command = std::str::from_utf8(&command.stdout)
            .expect("failed to parse stdout")
            .trim();

        println!("{:#?}", command.clone());

        let json: serde_json::Value = serde_json::from_str(command).unwrap();
        println!("{:#?}", json.clone());
        let json = json.as_object().unwrap();

        Feed {
            image: json["thumbnails"]
                .as_array()
                .unwrap()
                .into_iter()
                .rev()
                .find_map(|item| -> Option<String> {
                    let entry = item.as_object().unwrap();
                    if entry["id"].as_str() == Some("avatar_uncropped") {
                        Some(
                            entry["url"]
                                .as_str()
                                .expect("could not extract url as string for channel avatar")
                                .to_string(),
                        )
                    } else {
                        None
                    }
                })
                .unwrap(),
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
            .map(|itm| Episode::from(itm))
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
