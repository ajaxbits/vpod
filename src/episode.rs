use chrono::{Duration, NaiveDate};
use rss::{
    extension::{
        itunes::{ITunesItemExtension, ITunesItemExtensionBuilder},
        ExtensionBuilder,
    },
    GuidBuilder, Item, ItemBuilder,
};
use std::collections::BTreeMap;
use std::env;
use vpod::yt_xml::Video;

#[derive(Debug, Clone)]
pub struct Episode {
    id: rss::Guid,
    url: String,
    episode: Option<i32>,
    title: String,
    duration_str: String,
    duration_secs: i64,
    author: String,
    date: String,
    link: String,
    description: String,
}

pub fn gen_description(description: String) -> String {
    let description: String = description
        .split('\n')
        .into_iter()
        .map(|line| {
            format!(
                "<p>{}</p>",
                html_escape::encode_text_to_string(line, &mut String::new())
            )
        })
        .collect();
    description
}

impl Episode {
    pub fn new(
        id: String,
        title: String,
        duration: Duration,
        uploader: String,
        date: String,
        description: String,
    ) -> Self {
        fn gen_duration_str(duration: Duration) -> String {
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() - (&duration.num_hours() * 60);
            let seconds = duration.num_seconds() - (&duration.num_minutes() * 60);

            let time: Vec<String> = vec![hours, minutes, seconds]
                .into_iter()
                .map(|number| format!("{number}"))
                .map(|time_str| match time_str.chars().count() {
                    1 => format!("0{time_str}"),
                    _ => time_str,
                })
                .collect();

            format!("{}:{}:{}", time[0], time[1], time[2])
        }

        Episode {
            id: GuidBuilder::default()
                .value(id.clone())
                .permalink(false)
                .build(),
            url: format!(
                "{}/ep/{}",
                env::var("NGROK_URL").unwrap_or_else(|err| {
                    if err == env::VarError::NotPresent {
                        let app_name =
                            env::var("FLY_APP_NAME").expect("could not find $FLY_APP_NAME");
                        format!("https://{app_name}.fly.dev")
                    } else {
                        panic!("could not find $NGROK_URL or $FLY_APP_NAME in env");
                    }
                }),
                id
            ),
            episode: None,
            title,
            duration_str: gen_duration_str(duration),
            duration_secs: duration.num_seconds(),
            author: uploader,
            date,
            link: format!("https://www.youtube.com/watch?v={}", id),
            description,
        }
    }
    pub fn update_ep_number(self, number: i32) -> Self {
        Episode {
            episode: Some(number),
            ..self
        }
    }
}

impl From<Video> for Episode {
    fn from(video: Video) -> Self {
        Episode {
            id: GuidBuilder::default().value(&video.id.value).build(),
            url: format!(
                "{}/ep/{}",
                env::var("NGROK_URL").unwrap_or_else(|err| {
                    if err == env::VarError::NotPresent {
                        let app_name =
                            env::var("FLY_APP_NAME").expect("could not find $FLY_APP_NAME");
                        format!("https://{app_name}.fly.dev")
                    } else {
                        panic!("could not find $NGROK_URL or $FLY_APP_NAME in env");
                    }
                }),
                &video.id.value
            ),
            episode: None,
            title: video.title.value,
            duration_str: "00:30:00".to_string(),
            duration_secs: 1800,
            author: video.author.name.value,
            date: video.published.value.to_rfc2822(),
            link: video.link.href,
            description: video.group.description.value,
        }
    }
}

impl From<Item> for Episode {
    fn from(item: Item) -> Self {
        let itunes_info = item.itunes_ext().expect("item had no itunesextension");
        Episode {
            id: item.guid().expect("could not find id").to_owned(),
            url: item.link().expect("could not find link").to_owned(),
            episode: itunes_info.episode().map(|value| {
                value
                    .parse::<i32>()
                    .unwrap_or_else(|_| panic!("could not parse {value} as i32"))
            }),
            title: item
                .title()
                .expect("could not find title in feed")
                .to_owned(),
            duration_str: itunes_info
                .duration()
                .map(|v| v.to_owned())
                .expect("could not find duration for this episode"),
            duration_secs: item.enclosure()
                .map(|enc| enc.length())
                .map(|s| s.parse::<i64>().unwrap_or_else(|_| panic!("could not parse {s} as i64")))
                .expect("could not compute duration_secs from the enclosure for the specified rss entry"),
            author: item.author().expect("could not find author for specified episode").to_owned(),
            date: item.pub_date().expect("could not find date for specified episode").to_owned(),
            link: item.link().expect("could not find link for specified episode").to_owned(),
            description: item.description().expect("could not find description for specified episode").to_owned(),
        }
    }
}

impl From<youtube_dl::SingleVideo> for Episode {
    fn from(video: youtube_dl::SingleVideo) -> Self {
        let duration = video
            .duration
            .unwrap_or_else(|| panic!("could not find a duration for {}", video.id))
            .as_i64()
            .expect("could not convert duration to i64");
        let duration = Duration::seconds(duration);

        let date = NaiveDate::parse_from_str(
            &video
                .upload_date
                .unwrap_or_else(|| panic!("Could not find an upload_date for {}", video.id)),
            "%Y%m%d",
        )
        .map(|date| date.and_hms(0, 0, 0))
        .unwrap_or_else(|_| panic!("could not parse video {}'s upload date as str", video.id,));

        let date = chrono::DateTime::<chrono::Utc>::from_utc(date, chrono::Utc).to_rfc2822();

        Episode::new(
            video.id,
            video.title,
            duration,
            video.uploader.unwrap(),
            date,
            gen_description(video.description.unwrap()),
        )
    }
}

impl From<Episode> for rss::Item {
    fn from(ep: Episode) -> Self {
        let enclosure: rss::Enclosure = rss::EnclosureBuilder::default()
            .mime_type("audio/m4a".to_owned())
            .length(ep.duration_secs.to_string())
            .url(ep.url)
            .build();

        let itunes_metadata: ITunesItemExtension = ITunesItemExtensionBuilder::default()
            .episode(ep.episode.map(|ep| ep.to_string()))
            .author(Some(ep.author))
            .duration(Some(ep.duration_str))
            .block(Some("Yes".to_string()))
            .build();

        // We have to write a whole custom extension just to get <itunes:title>
        // TODO this is magic. Figure out how it works. Like what the heck are the
        // first entries in the BTreeMap about??
        let itunes_title = BTreeMap::from([(
            "itunes_title".to_owned(),
            vec![ExtensionBuilder::default()
                .name("itunes:title".to_owned())
                .value(Some(ep.title.clone()))
                .build()],
        )]);

        let item: Item = ItemBuilder::default()
            .guid(Some(ep.id))
            .pub_date(Some(ep.date))
            .title(Some(ep.title))
            .extensions(BTreeMap::from([("itunes_title".to_owned(), itunes_title)])) // put <itunes:title> in there
            .itunes_ext(Some(itunes_metadata))
            .enclosure(Some(enclosure))
            .link(Some(ep.link))
            .description(Some(ep.description))
            .build();

        item
    }
}
