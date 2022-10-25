use axum::body::StreamBody;
use futures::stream;
use tokio::process::Command;
use tokio_util::io::ReaderStream;
use ytd_rs::{error::YoutubeDLError, Arg, YoutubeDL};

pub fn stream_video_ytdl(channel_id: String) -> Result<Vec<SingleVideo>, YoutubeDLError> {
    let url = "https://www.youtube.com/watch?v=HMUugZ3DxH8";

    let command = Command::new("yt-dlp")
        .arg("--quiet")
        .arg("--format bestaudio[protocol^=http][abr<100][ext=m4a]")
        .arg("--output -")
        .spawn()
        .unwrap();

    let stdout = command.stdout.take().unwrap();
    let stream = ReaderStream::new(stdout);
    let body = StreamBody::new(stream);

    let path = PathBuf::from("./.");
    let ytd = YoutubeDL::new(&path, args, url).unwrap();

    let result = ytd.download().expect("youtube-dlp command failed");

    let result = result
        .output()
        .split_terminator("\n")
        .filter_map(|s| serde_json::from_str(s).ok())
        .collect::<Vec<SingleVideo>>();

    Ok(result)
}
