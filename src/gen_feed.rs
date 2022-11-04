use rss::Channel;
use std::collections::HashMap;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::process::Command;
use youtube_dl::model::SingleVideo;
use youtube_dl::Playlist;

use crate::episode::Episode;
use crate::feed::Feed;

fn get_recent_videos(cpfx: String, id: String) -> Vec<SingleVideo> {
    let link = format!("https://www.youtube.com/{cpfx}/{id}");

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

    videos
}

pub fn read_feed(channel_name: String) -> Result<Feed, rss::Error> {
    let file = format!("{channel_name}.xml");
    let file = File::open(file).map_err(|err| rss::Error::Xml(quick_xml::Error::Io(err)))?;
    let feed: Feed = rss::Channel::read_from(BufReader::new(file))?.into();

    Ok(feed)
}

pub async fn gen_feed(path: String, feed: Feed, cpfx: String, id: String) {
    let episodes: Vec<Episode> = get_recent_videos(cpfx, id)
        .into_iter()
        .map(Episode::from)
        .collect();
    let feed = Feed::add_episodes(feed, episodes);

    let channel = Channel::from(feed);
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .unwrap_or_else(|_| panic!("could not open {}", &path));
    channel.write_to(file).unwrap();

    let mut map = HashMap::new();
    map.insert(
        "urlprefix",
        env::var("NGROK_URL").unwrap_or_else(|err| {
            if err == env::VarError::NotPresent {
                let app_name = env::var("FLY_APP_NAME").expect("could not find $FLY_APP_NAME");
                format!("https://{app_name}.fly.dev")
            } else {
                panic!("could not find $NGROK_URL or $FLY_APP_NAME in env");
            }
        }),
    );

    //let client = reqwest::Client::new();
    //client
    //.post("https://overcast.fm/ping")
    //.json(&map)
    //.send()
    //.await
    //.unwrap();
}
