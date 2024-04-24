use crate::error::Result;
use axum::response::IntoResponse;

#[tracing::instrument]
pub(crate) async fn get_home() -> Result<impl IntoResponse> {
    tracing::info!("Got a homepage request");
    Ok("Welcome to my super cute home page")
}
