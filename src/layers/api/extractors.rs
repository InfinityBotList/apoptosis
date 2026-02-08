use crate::service::session::SessionPermit;

use super::server::{ApiError, ApiErrorCode, ApiResponseError, AppData};
use axum::extract::FromRequestParts;
use axum::Json;

/// This extractor checks if the user is authorized
/// from the DB and if so, returns the user id
pub struct AuthorizedSession(SessionPermit);

impl FromRequestParts<AppData> for AuthorizedSession {
    type Rejection = ApiResponseError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &AppData,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or_else(|| {
                (
                    axum::http::StatusCode::UNAUTHORIZED,
                    Json(ApiError {
                        message: "Whoa there! This endpoint requires authentication to use!"
                            .to_string(),
                        code: ApiErrorCode::NoAuthToken,
                    }),
                )
            })?;

        let permit = state.shared_layer.session_manager().get_permit_for(token)
            .await
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError {
                        message: format!("Failed to check auth for token due to error: {e:?}"),
                        code: ApiErrorCode::InternalAuthError,
                    }),
                )
            })?;

        match permit {
            SessionPermit::Success { .. } => Ok(AuthorizedSession(permit)),
            SessionPermit::InvalidToken => Err((
                axum::http::StatusCode::UNAUTHORIZED,
                Json(ApiError {
                    message:
                        "The token provided is invalid. Check that it hasn't expired and try again?"
                            .to_string(),
                    code: ApiErrorCode::InvalidToken,
                }),
            )),
            SessionPermit::ApiBanned { session } => Err((
                axum::http::StatusCode::FORBIDDEN,
                Json(ApiError {
                    message: format!(
                        "Target type {} with ID {} is banned from using the API",
                        session.target_type, session.target_id
                    ),
                    code: ApiErrorCode::ApiBanned,
                }),
            )),
            SessionPermit::EntityNotSupported => Err((
                axum::http::StatusCode::FORBIDDEN,
                Json(ApiError {
                    message: "The entity type associated with this session is not supported by the API".to_string(),
                    code: ApiErrorCode::Restricted,
                }),
            )),
        }
    }
}
