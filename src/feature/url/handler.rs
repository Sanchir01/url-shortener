use crate::metrics::PrometheusMetrics;
use axum::Extension;
use axum::extract::Path;
use axum::response::{IntoResponse, Redirect, Response};
use axum::{Json, extract::State, http::StatusCode};
use serde_json;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::feature::auth::entity::UserRole;
use crate::feature::url::entity::{CreateUrlDTO, RedirectDto};

use crate::servers::http::middleware::UserJWT;
use crate::{
    domain::url::Url,
    feature::url::service::{UrlService, UrlServiceTrait},
};

pub struct UrlHandler {
    url_service: Arc<UrlService>,
    metrics: Arc<PrometheusMetrics>,
}

impl UrlHandler {
    pub fn new_handler(url_service: Arc<UrlService>, metrics: Arc<PrometheusMetrics>) -> Self {
        Self {
            url_service,
            metrics,
        }
    }
}

#[utoipa::path(
    get,
    path = "/url",
    responses(
        (status = 200, description = "URLs retrieved successfully", body = Vec<Url>),
        (status = 401, description = "Unauthorized - requires authentication"),
        (status = 500, description = "Internal server error")
    ),
    tag = "URL"
)]
pub async fn get_all_url_handler_axum(
    State(handlers): State<Arc<UrlHandler>>,
) -> impl IntoResponse {
    match handlers.url_service.get_all_url().await {
        Ok(urls) => Ok(Json(urls)),
        Err(_) => {
            handlers.metrics.inc_errors("database_error", "url_handler");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error retrieving URLs".to_string()),
            ))
        }
    }
}

#[utoipa::path(
    post,
    path = "/url/save",
    request_body = CreateUrlDTO,
    responses(
        (status = 201, description = "URL created successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "URL"
)]
pub async fn create_url_handler(
    Extension(user_jwt): Extension<UserJWT>,
    State(handlers): State<Arc<UrlHandler>>,
    Json(payload): Json<CreateUrlDTO>,
) -> impl IntoResponse {
    let user = user_jwt.clone();
    if user.role != UserRole::Admin {
        handlers
            .metrics
            .inc_errors("authorization_error", "url_handler");
        return (StatusCode::FORBIDDEN, Json("Unauthorized".to_string()));
    }
    if let Err(validation_errors) = payload.validate() {
        handlers
            .metrics
            .inc_errors("validation_error", "url_handler");
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(format!("Validation error: {:?}", validation_errors)),
        );
    }

    match handlers.url_service.create_url(payload.url, user.id).await {
        Ok(_) => {
            handlers.metrics.inc_url_shortening();
            (StatusCode::CREATED, Json("Saved".to_string()))
        }
        Err(_) => {
            handlers.metrics.inc_errors("database_error", "url_handler");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error while saving".to_string()),
            )
        }
    }
}
#[utoipa::path(
    delete,
    path = "/url/{id}",
    params(
        ("id" = Uuid, Path, description = "URL ID to delete")
    ),
    responses(
        (status = 201, description = "URL deleted successfully"),
        (status = 401, description = "Unauthorized - requires authentication"),
        (status = 500, description = "Internal server error")
    ),
    security(
        ("cookie_auth" = [])
    ),
    tag = "URL"
)]
pub async fn delete_url_handler(
    State(handlers): State<Arc<UrlHandler>>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match handlers.url_service.delete_url(id).await {
        Ok(_) => (StatusCode::CREATED, Json("Deleted".to_string())),
        Err(_) => {
            handlers.metrics.inc_errors("database_error", "url_handler");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error while saving".to_string()),
            )
        }
    }
}

#[utoipa::path(
    delete,
    path = "/url/redirect",
    params(
        ("id" = Uuid, Path, description = "URL ID to delete")
    ),
    responses(
        (status = 201, description = "URL deleted successfully"),
        (status = 401, description = "Unauthorized - requires authentication"),
        (status = 500, description = "Internal server error")
    ),

    tag = "URL"
)]
pub async fn redirect_url_handler(
    State(handlers): State<Arc<UrlHandler>>,
    Json(payload): Json<RedirectDto>,
) -> Response {
    if let Err(validation_errors) = payload.validate() {
        handlers
            .metrics
            .inc_errors("validation_error", "url_handler");
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(format!("Validation error: {:?}", validation_errors)),
        )
            .into_response();
    }
    match handlers.url_service.get_url_by_hash(payload.id).await {
        Ok(Some(url)) => {
            handlers.metrics.inc_url_redirects();
            Redirect::permanent(&url.url).into_response()
        }
        Ok(None) => {
            handlers.metrics.inc_errors("not_found", "url_handler");
            (StatusCode::NOT_FOUND, Json("URL not found".to_string())).into_response()
        }
        Err(_) => {
            handlers.metrics.inc_errors("database_error", "url_handler");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Error while retrieving URL".to_string()),
            )
                .into_response()
        }
    }
}
