use axum::extract::Path;
use axum::response::Response;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

use crate::feature::url::entity::{CreateUrlDTO, DeleteUrlDto};
use crate::{
    domain::url::Url,
    feature::url::service::{UrlService, UrlServiceTrait},
};

pub struct UrlHandler {
    url_service: Arc<UrlService>,
}

impl UrlHandler {
    pub fn new_handler(url_service: Arc<UrlService>) -> Self {
        Self { url_service }
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
        Ok(urls) => {
            let body = serde_json::to_string(&urls).unwrap();
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(axum::body::Body::from(body))
                .unwrap()
        }
        Err(_) => {
            let body = serde_json::to_string(&Vec::<Url>::new()).unwrap();
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .header("content-type", "application/json")
                .body(body.into())
                .unwrap()
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
    State(handlers): State<Arc<UrlHandler>>,
    Json(payload): Json<CreateUrlDTO>,
) -> impl IntoResponse {
    if let Err(validation_errors) = payload.validate() {
        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(format!("Validation error: {:?}", validation_errors)),
        );
    }

    match handlers.url_service.create_url(payload.url).await {
        Ok(_) => (StatusCode::CREATED, Json("Saved".to_string())),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Error while saving".to_string()),
        ),
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
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json("Error while saving".to_string()),
        ),
    }
}
