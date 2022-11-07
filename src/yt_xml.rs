//TODO change this
#![allow(dead_code)]
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct XmlContentString {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct XmlContentu32 {
    #[serde(rename = "$value")]
    value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct XmlContentDateTime {
    #[serde(rename = "$value")]
    value: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Link {
    rel: String,
    href: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Author {
    name: XmlContentString,
    uri: XmlContentString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "feed")]
struct YtFeed {
    link: Vec<Link>,
    id: XmlContentString,
    #[serde(rename = "channelId")]
    channel_id: XmlContentString,
    title: XmlContentString,
    author: Author,
    published: XmlContentDateTime,

    entry: Vec<Video>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum Entry {
    Video(Video),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Video {
    id: XmlContentString,
    // #[serde(rename = "videoId")]
    video_id: XmlContentString,
    // #[serde(rename = "channelId")]
    channel_id: XmlContentString,
    title: XmlContentString,
    link: Link,
    author: Author,
    published: XmlContentDateTime,
    updated: XmlContentDateTime,
    group: MediaGroup,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaGroup {
    title: XmlContentString,
    content: MediaContent,
    thumbnail: MediaThumbnail,
    description: XmlContentString,
    community: MediaCommunity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaContent {
    url: String,
    r#type: String,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaThumbnail {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MediaCommunity {
    star_rating: MediaStarRating,
    statistics: MediaStatistics,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaStarRating {
    count: u32,
    average: Decimal,
    min: u32,
    max: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MediaStatistics {
    views: u64,
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use rust_decimal::Decimal;
    use serde::Deserialize;

    use super::YtFeed;

    #[test]
    fn test_basic_feed() {
        let vihart = include_str!("../test/vihart-one-entry.xml");
        let mut de = serde_xml_rs::Deserializer::new_from_reader(vihart.as_bytes())
            .non_contiguous_seq_elements(true);
        let vihart = YtFeed::deserialize(&mut de).unwrap();

        assert_eq!(vihart.id.value, "yt:channel:UCOGeU-1Fig3rrDjhm9Zs_wg");
        assert_eq!(vihart.channel_id.value, "UCOGeU-1Fig3rrDjhm9Zs_wg");
        assert_eq!(vihart.title.value, "Vihart");
        assert_eq!(vihart.author.name.value, "Vihart");
        assert_eq!(
            vihart.author.uri.value,
            "https://www.youtube.com/channel/UCOGeU-1Fig3rrDjhm9Zs_wg"
        );
        assert_eq!(
            vihart.published.value,
            DateTime::parse_from_rfc3339("2009-06-08T02:34:21+00:00").unwrap()
        );
    }
    #[test]
    fn test_single_entry() {
        let vihart = include_str!("../test/vihart-one-entry.xml");
        let mut de = serde_xml_rs::Deserializer::new_from_reader(vihart.as_bytes())
            .non_contiguous_seq_elements(true);
        let vihart = YtFeed::deserialize(&mut de).unwrap();
        let video = vihart.entry.into_iter().next().unwrap();
        let media = &video.group;

        assert_eq!(video.id.value, "yt:video:Twik7wqdwZU");
        assert_eq!(video.video_id.value, "Twik7wqdwZU");
        assert_eq!(video.channel_id.value, "UCOGeU-1Fig3rrDjhm9Zs_wg");
        assert_eq!(
            video.title.value,
            "only 0.000000001% of people will understand this video"
        );
        assert_eq!(video.link.rel, "alternate");
        assert_eq!(
            video.link.href,
            "https://www.youtube.com/watch?v=Twik7wqdwZU"
        );
        assert_eq!(video.author.name.value, "Vihart");
        assert_eq!(
            video.author.uri.value,
            "https://www.youtube.com/channel/UCOGeU-1Fig3rrDjhm9Zs_wg"
        );
        assert_eq!(
            video.published.value,
            DateTime::parse_from_rfc3339("2022-10-27T04:11:05+00:00").unwrap()
        );
        assert_eq!(
            video.updated.value,
            DateTime::parse_from_rfc3339("2022-10-27T06:56:21+00:00").unwrap()
        );

        assert_eq!(
            media.title.value,
            "only 0.000000001% of people will understand this video"
        );
        assert_eq!(
            media.content.url,
            "https://www.youtube.com/v/Twik7wqdwZU?version=3"
        );
        assert_eq!(media.content.r#type, "application/x-shockwave-flash");
        assert_eq!(media.content.width, 640);
        assert_eq!(media.content.height, 390);
        assert_eq!(
            media.thumbnail.url,
            "https://i1.ytimg.com/vi/Twik7wqdwZU/hqdefault.jpg"
        );
        assert_eq!(media.thumbnail.width, 480);
        assert_eq!(media.thumbnail.height, 360);
        assert_eq!(
            media.description.value,
            "Can you death-of-the-author a math test? How about cultural values? YouTube videos?\n\nThis video references Hank Green's video: https://youtu.be/lBJVyCYuu78\n\nHey, I made a video! Thank you patrons for supporting and encouraging me. I had fun with this one.\n\nUhh there is probably more to say but it's been a while and I forget how to youtube"
        );
        assert_eq!(media.community.star_rating.count, 12562);
        assert_eq!(
            media.community.star_rating.average,
            Decimal::from_str_exact("5.00").unwrap()
        );
        assert_eq!(media.community.star_rating.min, 1);
        assert_eq!(media.community.star_rating.max, 5);
        assert_eq!(media.community.statistics.views, 139833);
    }
}
