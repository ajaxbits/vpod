use serde::Deserialize;

#[derive(Deserialize)]
// #[serde(rename_all = "kebab-case")]
struct YtFeed {
    #[serde(alias = "@xmlns:yt")]
    yt_ns: String,
    #[serde(alias = "@xmlns:media")]
    media_ns: String,
    #[serde(alias = "@xmlns")]
    atom_ns: String,
    link: Vec<Link>,
    id: String,
    #[serde(alias = "yt:channelId")]
    yt_channel_id: String,
    title: String,
    author: Author,
    published: chrono::DateTime<Utc>,
    entry: Vec<Entry>,
}

struct Link {
    #[serde(alias = "@rel")]
    rel: String,
    #[serde(alias = "@href")]
    href: String,
}

struct Author {
    name: String,
    uri: String,
}

struct Entry {
    id: String,
    #[serde(alias = "yt:videoId")]
    video_id: String,
    #[serde(alias = "yt:channelId")]
    channel_id: String,
    title: String,
    link: Link,
    author: Author,
    published: chrono::DateTime<Utc>,
    updated: chrono::DateTime<Utc>,
    #[serde(alias = "media:group")]
    media_group: MediaGroup,
}

struct
