use rss::extension::itunes::{
    ITunesChannelExtensionBuilder, ITunesItemExtension, ITunesItemExtensionBuilder,
};
use rss::extension::ExtensionBuilder;
use rss::{ChannelBuilder, Enclosure, EnclosureBuilder, Guid, GuidBuilder, Item, ItemBuilder};
use std::collections::BTreeMap;
use std::fs::File;
use std::path::Path;

use youtube_dl::{YoutubeDl, YoutubeDlOutput};

use uuid::Uuid;

fn get_yt_link() -> String {
    let url = "https://www.youtube.com/watch?v=HMUugZ3DxH8";

    let path = Path::new(
        "/nix/store/r30s4a48b3n5icgmwgm4ds6a9yigffwq-python3.10-yt-dlp-2022.10.4/bin/yt-dlp",
    );

    let mut ytd = YoutubeDl::new(url);
    let ytd = ytd
        .socket_timeout("15")
        .youtube_dl_path(path)
        .format("bestaudio[protocol^=http][abr<40]");

    let ytd = ytd.run().expect("failed to run youtube-dlp command");

    if let YoutubeDlOutput::SingleVideo(boxed_video) = ytd {
        let video = *boxed_video;
        video.url.unwrap()
    } else {
        "https://www.google.com".to_string()
    }
}

fn build_episode() -> Item {
    let title = "Some Title".to_owned();

    let enclosure: Enclosure = EnclosureBuilder::default()
        .mime_type("audio/webm".to_owned())
        .length("SomeLengthInBytes".to_owned())
        .url(get_yt_link())
        .build();

    let guid: Guid = GuidBuilder::default()
        .value(Uuid::new_v4().as_simple().to_string())
        .permalink(false)
        .build();

    let itunes_metadata: ITunesItemExtension = ITunesItemExtensionBuilder::default()
        .episode(Some("1".to_owned()))
        .author(Some("Alex Jackson".to_owned()))
        .duration(Some("SomeDuration".to_owned()))
        .block(Some("Yes".to_string()))
        .build();

    // We have to write a whole custom extension just to get <itunes:title>
    // TODO this is magic. Figure out how it works. Like what the heck are the
    // first entries in the BTreeMap about??
    let itunes_title = BTreeMap::from([(
        "itunes_title".to_owned(),
        vec![ExtensionBuilder::default()
            .name("itunes:title".to_owned())
            .value(Some("this is the itunes title".to_owned()))
            .build()],
    )]);

    let item: Item = ItemBuilder::default()
        .guid(Some(guid))
        .pub_date(Some("DATE".to_owned()))
        .title(Some(title))
        .extensions(BTreeMap::from([("itunes_title".to_owned(), itunes_title)])) // put <itunes:title> in there
        .itunes_ext(Some(itunes_metadata))
        .enclosure(Some(enclosure))
        .link(Some("ogYoutubeLinkForCosmeticReasonsOnly".to_owned()))
        .description(Some("Some Description (goes in show notes)".to_string()))
        .build();

    item
}

fn main() {
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
        .items(vec![build_episode()])
        .build();

    let rss_file = File::create("test.xml").expect("could not create test.xml");

    channel
        .write_to(rss_file)
        .expect("could not write to rss_file");
}
