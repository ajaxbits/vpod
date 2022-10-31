use rss::extension::itunes::ITunesChannelExtensionBuilder;
use rss::{ChannelBuilder, Item};
use std::io::BufReader;
use std::process::Command;
use std::{collections::BTreeMap, fs::File};
use youtube_dl::model::SingleVideo;
use youtube_dl::Playlist;

use super::episode::Episode;
use super::feed::Feed;

fn get_recent_videos(channel_id: String) -> Vec<SingleVideo> {
    let link = format!("https://www.youtube.com/channel/{channel_id}");

    let command = Command::new("which").arg("yt-dlp").output().unwrap();

    let ytdlp_path = std::str::from_utf8(&command.stdout)
        .expect("failed to parse the stdout of the yt-dlp command")
        .trim();

    let output = youtube_dl::YoutubeDl::new(&link)
        .youtube_dl_path(ytdlp_path)
        .socket_timeout("30")
        .extra_arg("--playlist-end")
        .extra_arg("20")
        .run()
        .expect("Youtube-dlp command ran with errors");

    let videos: Playlist = match output {
        youtube_dl::YoutubeDlOutput::Playlist(playlist) => Some(*playlist),
        _ => None,
    }
    .expect("could not unwrap playlist item");

    let videos: Vec<SingleVideo> = videos
        .entries
        .expect("could not extract videos from Playlist item");

    // println!("{videos:#?}");

    videos
}

fn build_episode(video: SingleVideo) -> Item {
    let title = &video.title;
    let nv = video.clone();
}

fn read_feed(channel_name: String) -> Result<rss::Channel, rss::Error> {
    let file = format!("{channel_name}.xml");
    let file = File::open(file).map_err(|err| rss::Error::Xml(quick_xml::Error::Io(err)))?;
    let channel = rss::Channel::read_from(BufReader::new(file))?;
    Ok(channel)
}

fn gen_new_feed(current_feed: rss::Channel, new_items: Vec<Episode>) -> Vec<Episode> {
    let old_items: Vec<Episode> = current_feed.into_items().into();

    let old_ids = old_items
        .into_iter()
        .map(|item| item.guid.unwrap().value)
        .collect::<Vec<String>>();

    // FIXME this implementation is quadratic
    let new_episodes: Vec<Episode> = new_items
        .into_iter()
        .filter(|item| !old_ids.contains(&item.guid.value))
        .collect();

    // assuming that old_episodes is chronological, most-recent first
    let mut latest_number: i32 = old_items
        .clone()
        .into_iter()
        .next()
        .unwrap()
        .itunes_ext()
        .unwrap()
        .episode()
        .unwrap()
        .parse()
        .unwrap();

    let fixed_new_episodes: Vec<Episode> = new_episodes
        .into_iter()
        .rev()
        .map(|episode| {
            latest_number + 1;
            episode.update_ep_number(latest_number + 1)
        })
        .collect::<Vec<Episode>>()
        .into_iter()
        .chain()
        .collect();
}

pub async fn gen_feed(channel_id: String) {
    let channel_id = "UCNmv1Cmjm3Hk8Vc9kIgv0AQ".to_owned();
    let recents = get_recent_videos(channel_id);

    let latest: Episode = recents.clone().into_iter().nth(0).unwrap().into();

    let ep = build_episode(latest);
    let itunes_metadata = ITunesChannelExtensionBuilder::default()
        .author(Some("Alex Jackson".to_owned()))
        .build();

    let itunes_namespaces = BTreeMap::from([
        (
            "itunes".to_owned(),
            "http://www.itunes.com/dtds/podcast-1.0.dtd".to_owned(),
        ),
        (
            "content".to_owned(),
            "http://purl.org/rss/1.0/modules/content/".to_owned(),
        ),
    ]);

    let channel = ChannelBuilder::default()
        .namespaces(itunes_namespaces)
        .title("Test Title".to_owned())
        .link("http://test.com".to_owned())
        .description("A Test Feed".to_owned())
        .itunes_ext(Some(itunes_metadata))
        .items(vec![ep])
        .build();

    let rss_file = File::create("test.xml").expect("could not create test.xml");

    channel
        .write_to(rss_file)
        .expect("could not write to rss_file");
}
