use axum::response::IntoResponse;
use reqwest::StatusCode;

#[utoipa::path(
    get, 
    tag = "Public API",
    path = "/health-check",
    responses(
        (status = 204),
    )
)]
// super only exists inside api folder
pub(super) async fn health_check() -> impl IntoResponse {
    (StatusCode::NO_CONTENT, ())
}