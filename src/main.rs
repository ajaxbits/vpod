use rss::extension::itunes::{
    ITunesChannelExtensionBuilder, ITunesItemExtension, ITunesItemExtensionBuilder,
};
use rss::extension::ExtensionBuilder;
use rss::{ChannelBuilder, Enclosure, EnclosureBuilder, Guid, GuidBuilder, Item, ItemBuilder};
use std::{collections::BTreeMap, env, fs::File, path::PathBuf};
use uuid::Uuid;
use youtube_dl::model::SingleVideo;
use ytd_rs::{Arg, YoutubeDL};

fn get_yt_link() -> String {
    let url = "https://www.youtube.com/watch?v=HMUugZ3DxH8";
    let server_url = env::var("NGROK_URL").unwrap_or_else(|_err| {
        eprintln!("$NGROK_URL not found. Defaulting to localhost...");
        "127.0.0.1".to_string()
    });

    let args = vec![
        Arg::new("--quiet"),
        Arg::new_with_arg("--format", "bestaudio[protocol^=http][abr<100][ext=m4a]"),
        Arg::new_with_arg("--output", "%(uploader)s/%(id)s.%(ext)s"),
        Arg::new("--no-simulate"),
        Arg::new("--dump-json"),
    ];
    let path = PathBuf::from("./.");
    let ytd = YoutubeDL::new(&path, args, url).unwrap();

    let download = ytd.download().unwrap();

    let json: SingleVideo = serde_json::from_str(download.output()).unwrap();

    let creator: &str = &json.uploader.unwrap();
    let id: &str = &json.id;

    format!("{server_url}/{creator}/{id}.m4a")
}

fn build_episode() -> Item {
    let title = "Some Title".to_owned();

    let enclosure: Enclosure = EnclosureBuilder::default()
        .mime_type("audio/mp3".to_owned())
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
