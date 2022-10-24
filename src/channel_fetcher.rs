use std::path::PathBuf;

use youtube_dl::SingleVideo;
use ytd_rs::{error::YoutubeDLError, Arg, YoutubeDL};

pub fn get_recent_videos(channel_id: String) -> Result<Vec<SingleVideo>, YoutubeDLError> {
    let link = format!("https://www.youtube.com/channel/{channel_id}");

    let args = vec![
        Arg::new("--quiet"),
        Arg::new("--dump-json"),
        Arg::new_with_arg("--playlist-end", "20"),
    ];

    let result = YoutubeDL::new(&PathBuf::from("/tmp"), args, &link)?.download()?;

    let result = result
        .output()
        .split_terminator("\n")
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect::<Vec<SingleVideo>>();

    Ok(result)
}
