use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T, E = Report> = color_eyre::Result<T, E>;
pub struct Report(color_eyre::Report);

impl std::fmt::Debug for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for Report
where
    E: Into<color_eyre::Report>,
{
    fn from(e: E) -> Self {
        Self(e.into())
    }
}

impl IntoResponse for Report {
    fn into_response(self) -> Response {
        let e = self.0;
        let e_str = format!("{e:?}");

        tracing::error!("{e_str}");

        if let Some(e) = e.downcast_ref::<VpodError>() {
            return e.response();
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong".to_string(),
            )
                .into_response()
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum VpodError {
    #[error("could not find channel ID")]
    ChannelNotFound,
    #[error("playlist ID not found in url")]
    PlaylistIdNotFound,
    #[error("error running youtube-dlp")]
    YoutubeDLError,
}

impl VpodError {
    fn response(&self) -> Response {
        match self {
            Self::ChannelNotFound => (StatusCode::NOT_FOUND, "Channel not found").into_response(),
            Self::PlaylistIdNotFound => {
                (StatusCode::NOT_FOUND, "Channel not found").into_response()
            }
            Self::YoutubeDLError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Error getting audio").into_response()
            }
        }
    }
}
