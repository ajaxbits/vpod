use sqlx::{sqlite::SqliteQueryResult, SqlitePool};
use std::path::PathBuf;

use youtube_dl::SingleVideo;
use ytd_rs::{error::YoutubeDLError, Arg, YoutubeDL};

use crate::feed::Feed;

async fn write_update(
    channel_id: String,
    feed: &Feed,
    pool: SqlitePool,
) -> Result<SqliteQueryResult, sqlx::Error> {
    let now = chrono::Utc::now();
    let qry = "INSERT INTO updates (id,cadence,last_updated) VALUES ()";
    let result = sqlx::query(&qry).execute(&pool).await?;
    Ok(result)
}

async fn get_last_update(pool: SqlitePool) -> Result<SqliteQueryResult, sqlx::Error> {
    let qry = "SELECT id FROM updates";
    let result = sqlx::query(&qry).execute(&pool).await?;
    Ok(result)
}

pub fn get_videos(
    channel_id: String,
    number_videos: u32,
) -> Result<Vec<SingleVideo>, YoutubeDLError> {
    let link = format!("https://www.youtube.com/channel/{channel_id}");

    let args = vec![
        Arg::new("--quiet"),
        Arg::new("--dump-json"),
        Arg::new_with_arg("--playlist-end", &number_videos.to_string()),
    ];

    let result = YoutubeDL::new(&PathBuf::from("/tmp"), args, &link)?.download()?;

    let result = result
        .output()
        .split_terminator("\n")
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect::<Vec<SingleVideo>>();

    Ok(result)
}
