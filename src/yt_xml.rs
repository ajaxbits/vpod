//TODO change this
#![allow(dead_code)]
use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize)]
struct XmlContentString {
    #[serde(rename = "$value")]
    value: String,
}

#[derive(Deserialize)]
struct XmlContentu32 {
    #[serde(rename = "$value")]
    value: u32,
}

#[derive(Deserialize)]
struct XmlContentDateTime {
    #[serde(rename = "$value")]
    value: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize)]
struct Link {
    rel: String,
    href: String,
}

#[derive(Deserialize)]
struct Author {
    name: XmlContentString,
    uri: XmlContentString,
}

#[derive(Deserialize)]
#[serde(rename = "feed")]
struct YtFeed {
    #[serde(rename = "xmlns:yt")]
    yt_ns: String,
    #[serde(rename = "xmlns:media")]
    media_ns: String,
    #[serde(rename = "xmlns")]
    xml_ns: String,

    link: Link,
    id: XmlContentString,
    #[serde(rename = "yt:channelId")]
    yt_channel_id: XmlContentString,
    title: XmlContentString,
    author: Author,
    published: XmlContentDateTime,
    entry: Entries,
}

#[derive(Deserialize)]
struct Entries {
    #[serde(rename = "$value")]
    value: Vec<Entry>,
}

#[derive(Deserialize)]
enum Entry {
    Video(Video),
}

#[derive(Deserialize)]
struct Video {
    id: XmlContentString,
    #[serde(rename = "yt:videoId")]
    video_id: XmlContentString,
    #[serde(rename = "yt:channelId")]
    channel_id: XmlContentString,
    title: XmlContentString,
    link: Link,
    author: Author,
    published: XmlContentDateTime,
    updated: XmlContentDateTime,
    #[serde(rename = "media:group")]
    media_group: MediaGroup,
}

#[derive(Deserialize)]
struct MediaGroup {
    #[serde(rename = "media:title")]
    title: XmlContentString,
    #[serde(rename = "media:content")]
    content: MediaContent,
    #[serde(rename = "media:thumbnail")]
    thumbnail: MediaThumbnail,
    #[serde(rename = "media:description")]
    description: XmlContentString,
    #[serde(rename = "media:community")]
    community: MediaCommunity,
}

#[derive(Deserialize)]
struct MediaContent {
    url: String,
    #[serde(rename = "type")]
    content_type: String,
    width: u32,
    height: u32,
}

#[derive(Deserialize)]
struct MediaThumbnail {
    url: String,
    width: u32,
    height: u32,
}

#[derive(Deserialize)]
struct MediaCommunity {
    #[serde(rename = "media:starRating")]
    star_rating: MediaStarRating,
    #[serde(rename = "media:statistics")]
    statistics: MediaStatistics,
}

#[derive(Deserialize)]
struct MediaStarRating {
    count: u32,
    average: Decimal,
    min: u32,
    max: u32,
}

#[derive(Deserialize)]
struct MediaStatistics {
    views: u64,
}

#[cfg(test)]
mod tests {
    use serde_xml_rs::from_str;

    use super::YtFeed;

    #[test]
    fn test_vihart() {
        let vihart = include_str!("../test/vihart-one-entry.xml");
        let vihart: YtFeed = from_str(vihart).expect("could not parse vihart-one-entry.xml");

        assert_eq!(vihart.yt_ns, "http://www.youtube.com/xml/schemas/2015");
        assert_eq!(vihart.media_ns, "http://search.yahoo.com/mrss/");
        assert_eq!(vihart.xml_ns, "http://www.w3.org/2005/Atom");
    }
}
