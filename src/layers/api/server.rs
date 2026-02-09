use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
//use std::sync::Arc;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::openapi::server::Server;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;
use super::public_api;

use crate::service::sharedlayer::SharedLayer;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub enum ApiErrorCode {
    InternalAuthError,
    NoAuthToken,
    ApiBanned,
    InvalidToken,
    InternalError,
    Restricted,
    NotFound,
    BadRequest,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ApiError {
    pub message: String,
    pub code: ApiErrorCode,
}

impl From<String> for ApiError {
    fn from(message: String) -> Self {
        ApiError {
            message,
            code: ApiErrorCode::InternalError,
        }
    }
}

impl<'a> From<&'a str> for ApiError {
    fn from(message: &'a str) -> Self {
        log::error!("Returning error: {message}");
        ApiError {
            message: message.to_string(),
            code: ApiErrorCode::InternalError,
        }
    }
}

#[derive(Clone)]
pub struct AppData {
    pub shared_layer: SharedLayer,
}

impl AppData {
    pub fn new(shared_layer: SharedLayer) -> Self {
        Self { shared_layer }
    }
}

pub type ApiResponseError = (StatusCode, Json<ApiError>);
pub type ApiResponse<T> = Result<Json<T>, ApiResponseError>;

async fn logger(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    log::info!(
        "Received request: method = {}, path={}",
        request.method(),
        request.uri().path()
    );

    let response = next.run(request).await;
    response
}

pub fn create(
    shared: SharedLayer,
) -> axum::routing::IntoMakeService<Router> {
    let mut router = Router::new();

    // Public routes
    let public_routes = [
        routes!(public_api::health_check),
    ];

    let mut oapi_router = OpenApiRouter::new();
    for route in public_routes {
        oapi_router = oapi_router.routes(route.clone());

        let mut paths = route.1.paths;
        let path = {
            assert!(paths.len() == 1, "Public API routes should have one path");
            let first_entry = paths.first_entry();
            let path = first_entry.map(|path| path.key().clone()).unwrap();
            if path.is_empty() {
                "/".to_string()
            } else {
                path
            }
        };

        router = router.route(&path, route.2);
    }

    let mut public_openapi = oapi_router.into_openapi();

    // Set OpenAPI servers so Swagger UI knows the API base URL
    public_openapi.servers = Some(vec![Server::new("https://spider-staging.omniplex.gg")]);

    // Add PublicAuth
    if let Some(comps) = public_openapi.components.as_mut() {
        comps.security_schemes.insert(
            "PublicAuth".to_string(),
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "Authorization",
                "API token. This API is public but requires authentication",
            ))),
        );
    }

    router = router
        .route("/healthcheck", post(|| async { Json(()) }))
        .merge(SwaggerUi::new("/docs").url("/openapi", public_openapi))
        .fallback(get(|| async {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError {
                    message: "Not Found".to_string(),
                    code: ApiErrorCode::NotFound,
                }),
            )
        }))
        .layer(tower_http::cors::CorsLayer::very_permissive())
        .layer(axum::middleware::from_fn(logger));

    let router: Router<()> = router.with_state(AppData::new(shared));
    router.into_make_service()
}