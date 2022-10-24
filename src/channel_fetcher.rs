use futures::StreamExt;
use ytextract::playlist;

async fn fetch_info(id: String) -> Result<Vec<playlist::Video>, Box<dyn std::error::Error>> {
    let ytclient = ytextract::Client::new();
    let id: ytextract::channel::Id = id.parse()?;
    let channel = ytclient.channel(id).await?;
    let uploads = channel.uploads().await?;
    let recents: Vec<playlist::Video> = uploads
        .take(10)
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .filter_map(|video| video.ok())
        .collect();

    Ok(recents)
}

pub async fn get_recent_links(channel_id: String) -> Vec<String> {
    let links = self::fetch_info(channel_id).await.unwrap();
    links
        .into_iter()
        .map(|video| {
            let video_id = video.id().to_string();
            format!("https://www.youtube.com/watch?v={video_id}")
        })
        .collect()
}
