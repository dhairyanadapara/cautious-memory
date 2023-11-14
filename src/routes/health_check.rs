use axum::http::StatusCode;

#[tracing::instrument]
pub fn health_check() -> StatusCode {
    StatusCode::NO_CONTENT
}
