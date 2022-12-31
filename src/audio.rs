use std::path::PathBuf;

use axum::{response::IntoResponse, routing::get_service};
use tower::ServiceExt;
use ytd_rs::Arg;

// #[axum::debug_handler]
pub async fn return_audio(
    axum::extract::Path(id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let url = format!("https://www.youtube.com/watch?v={id}");
    let path = format!("{id}.m4a");
    if let false = std::path::Path::new(&path).exists() {
        let args = vec![
            Arg::new("--quiet"),
            Arg::new_with_arg("--format", "bestaudio[protocol^=http][abr<100][ext=m4a]"),
            Arg::new("--embed-metadata"),
            Arg::new("--embed-thumbnail"),
            Arg::new_with_arg("--sponsorblock-mark", "sponsor,selfpromo"),
            Arg::new_with_arg("--output", "%(id)s.m4a"),
        ];
        let _ytd = ytd_rs::YoutubeDL::new(&PathBuf::from("./."), args, &url)
            .unwrap()
            .download();
    }

    let req = hyper::Request::builder()
        .uri(id)
        .body(axum::body::Body::empty())
        .unwrap();

    let service =
        get_service(tower_http::services::ServeFile::new(path)).handle_error(crate::handle_error);

    let result = service.oneshot(req).await;

    result
}
