use std::{env, path::PathBuf};

use axum::{response::IntoResponse, routing::get_service};
use tower::ServiceExt;
use ytd_rs::Arg;

// #[axum::debug_handler]
pub async fn return_audio(
    axum::extract::Path((feed_id, ep_id)): axum::extract::Path<(String, String)>,
) -> impl IntoResponse {
    let url = format!("https://www.youtube.com/watch?v={ep_id}");
    let path = format!("{feed_id}/{ep_id}.m4a");
    let path = std::path::Path::new(&path);
    if let false = &path.exists() {
        let args = vec![
            Arg::new("--quiet"),
            Arg::new_with_arg("--format", "bestaudio[protocol^=http][abr<100][ext=m4a]"),
            Arg::new("--embed-metadata"),
            Arg::new("--embed-thumbnail"),
            Arg::new_with_arg("--sponsorblock-mark", "sponsor,selfpromo"),
            Arg::new_with_arg("--output", "%(id)s.m4a"),
        ];
        let _ytd = ytd_rs::YoutubeDL::new(&PathBuf::from(&path.parent().unwrap()), args, &url)
            .unwrap()
            .download();

        let target_dir_size = env::var("TARGET_DIR_SIZE").unwrap_or("100000".to_string());
        let target_dir_size: u64 = target_dir_size.parse::<u64>().unwrap();
        let dir = &path.parent().unwrap();
        let dir_size: u64 = fs_extra::dir::get_size(dir).unwrap() / 1000; //Kb
        let difference: i64 = dir_size as i64 - target_dir_size as i64;
        println!("{dir_size}");

        if difference >= 0 {
            let mut m4a_files: Vec<PathBuf> = std::fs::read_dir(dir)
                .unwrap()
                .filter_map(|entry| entry.ok())
                .map(|entry| entry.path())
                .filter(|path| path.extension().map_or(false, |ext| ext == "m4a"))
                .collect();
            m4a_files.sort_by(|a, b| {
                a.metadata()
                    .unwrap()
                    .modified()
                    .unwrap()
                    .cmp(&b.metadata().unwrap().modified().unwrap())
            });

            let mut difference = difference;
            println!("{difference}");

            while difference >= 0 {
                let oldest_file = &m4a_files[0];
                difference = difference - ((oldest_file.metadata().unwrap().len() / 1000) as i64);
                println!("{difference}");
                // std::fs::remove_file(&m4a_files[0]).expect("could not delete file");
            }
        }
    }

    let req = hyper::Request::builder()
        .uri(ep_id)
        .body(axum::body::Body::empty())
        .unwrap();

    let service =
        get_service(tower_http::services::ServeFile::new(path)).handle_error(crate::handle_error);

    let result = service.oneshot(req).await;

    result
}
