use crate::feature::auth::handler::{
    get_user_by_email_handler, google_oauth_handler, handle_google_code, register_handler,
};
use crate::feature::url::handler::{create_url_handler, delete_url_handler};
use crate::{ApiDoc, app::handlers::Handlers, feature::url::handler::get_all_url_handler_axum};
use axum::{
    Router,
    routing::{delete, get, post},
};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run_http_server(host: &str, port: u16, handlers: Arc<Handlers>) {
    let listener = TcpListener::bind(format!("{}:{}", host, port))
        .await
        .unwrap();
    let auth_google = Router::new()
        .route("/url", get(google_oauth_handler))
        .route("/callback", post(handle_google_code));
    let auth_basic = Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(get_user_by_email_handler))
        .with_state(handlers.user_handle.clone());
    let public_routes = Router::new()
        .route("/url", get(get_all_url_handler_axum))
        .route("/url/save", post(create_url_handler))
        .route("/url/{id}", delete(delete_url_handler))
        .nest("/auth/google", auth_google)
        .nest("/auth", auth_basic)
        .with_state(handlers.url_handler.clone());

    let app = axum::Router::new()
        .nest("/api", public_routes)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(get_cors());

    axum::serve(listener, app).await.unwrap();
}
fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}
