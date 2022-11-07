use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename = "feed")]
pub struct YtFeed {
    pub link: Vec<Link>,
    pub id: XmlContentString,
    #[serde(rename = "channelId")]
    pub channel_id: XmlContentString,
    pub title: XmlContentString,
    pub author: Author,
    pub published: XmlContentDateTime,
    #[serde(rename = "entry")]
    pub videos: Vec<Video>,
}

impl YtFeed {
    pub async fn from_channel_id(id: &str) -> Self {
        let url = format!("https://www.youtube.com/feeds/videos.xml?channel_id={id}");
        let resp = reqwest::get(url).await.unwrap();
        let xml = resp.text().await.unwrap();
        let mut de = serde_xml_rs::Deserializer::new_from_reader(xml.as_bytes())
            .non_contiguous_seq_elements(true);

        YtFeed::deserialize(&mut de).unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmlContentString {
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmlContentu32 {
    #[serde(rename = "$value")]
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct XmlContentDateTime {
    #[serde(rename = "$value")]
    pub value: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Link {
    pub rel: String,
    pub href: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Author {
    pub name: XmlContentString,
    pub uri: XmlContentString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    pub id: XmlContentString,
    pub video_id: XmlContentString,
    pub channel_id: XmlContentString,
    pub title: XmlContentString,
    pub link: Link,
    pub author: Author,
    pub published: XmlContentDateTime,
    pub updated: XmlContentDateTime,
    pub group: MediaGroup,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaGroup {
    pub title: XmlContentString,
    pub content: MediaContent,
    pub thumbnail: MediaThumbnail,
    pub description: XmlContentString,
    pub community: MediaCommunity,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaContent {
    pub url: String,
    pub r#type: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaThumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MediaCommunity {
    pub star_rating: MediaStarRating,
    pub statistics: MediaStatistics,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaStarRating {
    pub count: u32,
    pub average: Decimal,
    pub min: u32,
    pub max: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MediaStatistics {
    pub views: u64,
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use rust_decimal::Decimal;
    use serde::Deserialize;

    use super::YtFeed;

    #[test]
    fn test_basic_feed() {
        let vihart = include_str!("../test/vihart.xml");
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
        let vihart = include_str!("../test/vihart.xml");
        let mut de = serde_xml_rs::Deserializer::new_from_reader(vihart.as_bytes())
            .non_contiguous_seq_elements(true);
        let vihart = YtFeed::deserialize(&mut de).unwrap();
        let video = vihart.videos.into_iter().next().unwrap();
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

    #[test]
    fn test_entry_amounts() {
        let vihart = include_str!("../test/vihart.xml");
        let mut de = serde_xml_rs::Deserializer::new_from_reader(vihart.as_bytes())
            .non_contiguous_seq_elements(true);
        let vihart = YtFeed::deserialize(&mut de).unwrap();
        assert_eq!(vihart.videos.iter().count(), 15);
    }

    #[tokio::test]
    async fn test_grim_beard_id() {
        let url = "https://www.youtube.com/feeds/videos.xml?channel_id=UCNmv1Cmjm3Hk8Vc9kIgv0AQ";
        let resp = reqwest::get(url).await.unwrap();
        let xml = resp.text().await.unwrap();
        let mut de = serde_xml_rs::Deserializer::new_from_reader(xml.as_bytes())
            .non_contiguous_seq_elements(true);

        let grim_beard = YtFeed::deserialize(&mut de).unwrap();
        assert_eq!(grim_beard.videos.iter().count(), 15);
        assert_eq!(grim_beard.id.value, "yt:channel:UCNmv1Cmjm3Hk8Vc9kIgv0AQ");
        assert_eq!(grim_beard.channel_id.value, "UCNmv1Cmjm3Hk8Vc9kIgv0AQ");
        assert_eq!(grim_beard.title.value, "Grim Beard");
        assert_eq!(grim_beard.author.name.value, "Grim Beard");
        assert_eq!(
            grim_beard.author.uri.value,
            "https://www.youtube.com/channel/UCNmv1Cmjm3Hk8Vc9kIgv0AQ"
        );
        assert_eq!(
            grim_beard.published.value,
            DateTime::parse_from_rfc3339("2013-08-14T03:37:55+00:00").unwrap()
        );
    }
}
