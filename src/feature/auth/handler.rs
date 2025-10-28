use crate::feature::auth::entity::{AuthGoogleDTO, LoginDTO, RegisterDTO};
use crate::feature::auth::jwt::set_jwt;
use crate::feature::auth::service::{UserService, UserServiceTrait};
use crate::utils::url::generate_google_oauth_url;
use axum::{
    Json as AxumJson,
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
};
use reqwest::ClientBuilder;
use serde_json::json;
use std::env;
use std::sync::Arc;
use validator::Validate;

pub struct UserHandler {
    user_service: Arc<UserService>,
}

impl UserHandler {
    pub fn new_handler(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }
}

#[utoipa::path(
    get,
    path = "/auth/google/url",
    responses(
        (status = 302, description = "Redirect to Google OAuth"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn google_oauth_handler() -> Result<Redirect, (StatusCode, Json<serde_json::Value>)> {
    match generate_google_oauth_url() {
        Ok(url) => Ok(Redirect::temporary(&url)),
        Err(e) => {
            eprintln!("Ошибка генерации Google OAuth URL: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "ошибка на сервере" })),
            ))
        }
    }
}

#[utoipa::path(
    post,
    path = "/auth/google/callback",
    request_body = AuthGoogleDTO,
    responses(
        (status = 200, description = "Google OAuth callback successful"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error"),
        (status = 502, description = "Bad gateway")
    ),
    tag = "Auth"
)]
pub async fn handle_google_code(
    Json(payload): Json<AuthGoogleDTO>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "error": validation_errors
            })),
        ));
    }

    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("Failed to build client: {}", e)})),
            )
        })?;
    let params = [
        ("code", payload.code.clone()),
        ("client_id", env::var("GOOGLE_CLIENT_ID").unwrap()),
        ("client_secret", env::var("GOOGLE_SECRET").unwrap()),
        ("redirect_uri", env::var("GOOGLE_URI_REDIRECT").unwrap()),
        ("grant_type", "authorization_code".to_string()),
    ];
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| {
            (
                StatusCode::BAD_GATEWAY,
                Json(json!({ "error": format!("Request failed: {}", e) })),
            )
        })?;

    let res_json = res.json().await.map_err(|e| {
        (
            StatusCode::BAD_GATEWAY,
            Json(json!({"error": format!("Invalid JSON: {}", e)})),
        )
    })?;

    println!("Ответ от Google: {:?}", res_json);
    Ok(Json(res_json))
}

#[utoipa::path(
    post,
    path = "/auth/register",
    request_body =RegisterDTO,
    responses(
        (status = 201, description = "URL created successfully"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error")
    ),
    tag = "Auth"
)]
pub async fn register_handler(
    State(handler): State<Arc<UserHandler>>,
    Json(payload): Json<RegisterDTO>,
) -> impl IntoResponse {
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            AxumJson(json!({
                "error": "Validation error",
                "details": validation_errors
            })),
        ));
    }
    match handler
        .user_service
        .create_user_service(payload.title, payload.email, payload.password)
        .await
    {
        Ok(user) => {
            let cookies = match set_jwt(user.id, user.role.clone()).await {
                Ok(jar) => jar,
                Err(e) => {
                    eprintln!("❌ JWT generation error: {}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        AxumJson(json!({"error": "Failed to create session"})),
                    ));
                }
            };

            Ok((
                StatusCode::CREATED,
                (
                    cookies,
                    AxumJson(json!({
                        "message": "User created",
                        "user": user
                    })),
                ),
            ))
        }
        Err(e) => {
            eprintln!("❌ Internal error: {:?}", e);
            Err((
                StatusCode::FORBIDDEN,
                AxumJson(json!({"error": "Error creating user"})),
            ))
        }
    }
}

#[utoipa::path(
    post,
    path = "/auth/login",
    request_body = LoginDTO,
    responses(
        (status = 200, description = "Login successful"),
        (status = 422, description = "Validation error"),
        (status = 500, description = "Internal server error"),
        (status = 502, description = "Bad gateway")
    ),
    tag = "Auth"
)]
pub async fn get_user_by_email_handler(
    State(handler): State<Arc<UserHandler>>,
    Json(payload): Json<LoginDTO>,
) -> impl IntoResponse {
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::UNPROCESSABLE_ENTITY,
            AxumJson(json!({
                "error": "Validation error",
                "details": validation_errors
            })),
        ));
    }
    match handler
        .user_service
        .get_user_by_email_service(payload.email, payload.password)
        .await
    {
        Ok(Some(user)) => match set_jwt(user.id, user.role.clone()).await {
            Ok(cookies) => Ok((StatusCode::OK, (cookies, AxumJson(json!({ "user": user }))))),
            Err(err) => {
                eprintln!("❌ JWT generation error: {err}");
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    AxumJson(json!({ "error": "Failed to create session" })),
                ))
            }
        },
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            AxumJson(json!({ "error": "User not found" })),
        )),
        Err(e) => {
            eprintln!("❌ Internal error: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                AxumJson(json!({ "error": "Internal server error" })),
            ))
        }
    }
}
