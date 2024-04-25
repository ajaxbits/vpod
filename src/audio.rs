use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::error::{Result, VpodError};
use axum::response::IntoResponse;
use color_eyre::eyre::eyre;
use tower::ServiceExt;
use ytd_rs::Arg;

#[tracing::instrument(fields(feed_id=feed_id, episode_id=file_name))]
pub async fn return_audio(
    axum::extract::Path((feed_id, file_name)): axum::extract::Path<(String, String)>,
    request: axum::extract::Request,
) -> Result<impl IntoResponse> {
    let ep_id = std::path::Path::new(&file_name)
        .file_stem()
        .ok_or(eyre!("could not get file stem for episode"))?
        .to_str()
        .ok_or(eyre!("could not format episode file id to str"))?;
    let url = format!("https://www.youtube.com/watch?v={ep_id}");
    let path = format!("{feed_id}/{file_name}");
    let path = std::path::Path::new(&path);
    if !path.exists() {
        let args = vec![
            // TODO: Implement an enum allowing users to safely
            // add their own options to this list
            Arg::new("--quiet"),
            Arg::new_with_arg("--concurrent-fragments", "8"),
            Arg::new_with_arg("--format", "bestaudio[protocol^=http][abr<100][ext=m4a]"),
            Arg::new("--embed-metadata"),
            Arg::new("--embed-thumbnail"),
            Arg::new_with_arg("--sponsorblock-mark", "sponsor,selfpromo"),
            Arg::new_with_arg("--output", "%(id)s.m4a"),
        ];
        let channel_dir = &PathBuf::from(&path.parent().unwrap());
        let _ytd = ytd_rs::YoutubeDL::new(channel_dir, args, &url)
            .map_err(|_| VpodError::YoutubeDLError)?
            .download();

        let target_dir_size = env::var("TARGET_DIR_SIZE").unwrap_or("100000".to_string());
        let target_dir_size: u64 = target_dir_size.parse::<u64>().unwrap();
        let dir = path.parent().unwrap();

        // Call to the new function
        match reduce_dir_size(dir, target_dir_size) {
            Err(e) => eprintln!("Failed to reduce directory size: {:?}", e),
            _ => (),
        }
    }

    let service = tower_http::services::ServeFile::new(path);

    let result = service.oneshot(request).await;

    Ok(result)
}

#[tracing::instrument]
fn reduce_dir_size(dir: &Path, target_dir_size: u64) -> Result<()> {
    let dir_size: u64 = fs_extra::dir::get_size(dir)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
        / 1000; //Kb
    let mut difference: i64 = dir_size as i64 - target_dir_size as i64;

    if difference >= 0 {
        let mut m4a_files: Vec<PathBuf> = fs::read_dir(dir)?
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().map_or(false, |ext| ext == "m4a"))
            .collect();

        m4a_files.sort_by_key(|a| a.metadata().unwrap().modified().unwrap());

        while difference >= 0 && !m4a_files.is_empty() {
            let oldest_file = m4a_files.remove(0);
            difference -= (oldest_file.metadata()?.len() / 1000) as i64;
            fs::remove_file(&oldest_file)?;
        }
    }

    Ok(())
}
