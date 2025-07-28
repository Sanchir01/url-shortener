use crate::feature::auth::handler::{
    get_user_by_email_handler, google_oauth_handler, handle_google_code, register_handler,
};
use crate::feature::url::handler::{create_url_handler, delete_url_handler};
use crate::servers::http::middleware::auth_middleware;
use crate::{
    app::handlers::Handlers, feature::url::handler::get_all_url_handler_axum,
    swagger::swagger_api::ApiDoc,
};
use axum::{
    Router,
    middleware::from_fn,
    routing::{delete, get, post},
};
use sqlx::{Pool, Postgres};
use tower_http::compression::CompressionLayer;

use std::sync::Arc;
#[cfg(unix)]
use tokio::signal::unix::SignalKind;
use tokio::{net::TcpListener, signal};
use tower_http::cors::{Any, CorsLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub async fn run_http_server(host: &str, port: u16, handlers: Arc<Handlers>, pool: Pool<Postgres>) {
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

    let private_router = Router::new()
        .route("/url/save", post(create_url_handler))
        .route("/url/{id}", delete(delete_url_handler))
        .with_state(handlers.url_handler.clone())
        .layer(from_fn(auth_middleware));

    let public_routes = Router::new()
        .route("/url", get(get_all_url_handler_axum))
        .nest("/auth/google", auth_google)
        .nest("/auth", auth_basic)
        .with_state(handlers.url_handler.clone());

    let app = axum::Router::new()
        .nest("/api", public_routes)
        .nest("/api/private", private_router)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(get_cors())
        .layer(CompressionLayer::new());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(pool))
        .await
        .unwrap();
}
fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

pub async fn shutdown_signal(pool: Pool<Postgres>) {
    #[cfg(unix)]
    {
        let mut sigint = signal(SignalKind::interrupt()).expect("failed to bind SIGINT handler");
        let mut sigterm = signal(SignalKind::terminate()).expect("failed to bind SIGTERM handler");

        tokio::select! {
            _ = sigint.recv() => {
                println!("🔌 Received SIGINT (Ctrl+C)");
            },
            _ = sigterm.recv() => {
                println!("🛑 Received SIGTERM (kill)");
            },
        }

        pool.close().await;
        println!("✅ Pool closed gracefully");
    }

    #[cfg(not(unix))]
    {
        signal::ctrl_c().await.expect("failed to listen for Ctrl+C");
        println!("🪟 Received Ctrl+C (Windows)");
        pool.close().await;
        println!("✅ Pool closed gracefully");
    }
}
