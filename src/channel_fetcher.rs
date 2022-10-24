use futures::StreamExt;
use ytextract::playlist;

pub async fn fetch_info(id: String) -> Result<Vec<playlist::Video>, Box<dyn std::error::Error>> {
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
