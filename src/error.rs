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
    #[error("A spooky thing happened")]
    Spooky,
}

impl VpodError {
    fn response(&self) -> Response {
        match self {
            Self::Spooky => (StatusCode::IM_A_TEAPOT, "This is a test error").into_response(),
        }
    }
}
