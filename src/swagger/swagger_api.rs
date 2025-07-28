use crate::domain::url::Url;
use crate::feature::auth::entity::{AuthGoogleDTO, LoginDTO, RegisterDTO};
use crate::feature::url::entity::CreateUrlDTO;
use utoipa::OpenApi;

#[derive(utoipa::ToSchema)]
pub struct CookieAuth;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Url shortener",
        version = "0.0.1",
        description = "API для сокращения URL и аутентификации через Google OAuth"
    ),
    paths(
        crate::feature::url::handler::get_all_url_handler_axum,
        crate::feature::url::handler::create_url_handler,
        crate::feature::url::handler::delete_url_handler,
        crate::feature::auth::handler::google_oauth_handler,
        crate::feature::auth::handler::handle_google_code,
        crate::feature::auth::handler::register_handler,
        crate::feature::auth::handler::get_user_by_email_handler
    ),
    components(
        schemas(CreateUrlDTO, AuthGoogleDTO, Url, CookieAuth, RegisterDTO,LoginDTO)
    ),
    tags(
        (name = "URL", description = "Операции с URL"),
        (name = "Auth", description = "Аутентификация через Google OAuth и почту с паролем")
    ),
    servers(
        (url = "/api", description = "API base path")
    )
)]
pub struct ApiDoc;
