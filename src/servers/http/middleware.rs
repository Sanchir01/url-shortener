use crate::feature::auth::entity::UserRole;
use crate::feature::auth::jwt::decode_jwt;
use crate::utils::constants::{ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE};
use axum::{
    extract::Request,
    http::{StatusCode, header::COOKIE},
    middleware::Next,
    response::Response,
};
use cookie::Cookie;
use log::{error, info};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RefreshToken(pub String);

#[derive(Debug, Clone)]
pub struct AccessToken(pub String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserJWT {
    pub id: Uuid,
    pub role: UserRole,
}

pub async fn auth_middleware(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    if let Some(tokens) = extract_tokens_from_request(&req) {
        if let Some(access_token) = tokens.access_token {
            match decode_jwt(&access_token).await {
                Ok(user_jwt) => {
                    req.extensions_mut().insert(UserJWT {
                        id: user_jwt.id,
                        role: user_jwt.role,
                    });
                    info!("Valid access token for user:",);
                }
                Err(e) => {
                    error!("Invalid access token: {:?}", e);
                }
            }
        }

        if let Some(refresh_token) = tokens.refresh_token {
            match decode_jwt(&refresh_token).await {
                Ok(user_jwt) => {
                    req.extensions_mut().insert(UserJWT {
                        id: user_jwt.id,
                        role: user_jwt.role,
                    });
                    info!("Valid access token for user");
                }
                Err(e) => {
                    error!("Invalid access token: {:?}", e);
                }
            }
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}

struct Tokens {
    access_token: Option<String>,
    refresh_token: Option<String>,
}

fn extract_tokens_from_request(req: &Request) -> Option<Tokens> {
    let cookie_header = req.headers().get(COOKIE)?;
    let cookie_str = cookie_header.to_str().ok()?;

    let cookies: Vec<Cookie> = cookie_str
        .split(';')
        .filter_map(|cookie| Cookie::parse(cookie.trim()).ok())
        .collect();

    let access_token = cookies
        .iter()
        .find(|cookie| cookie.name() == ACCESS_TOKEN_COOKIE)
        .map(|cookie| cookie.value().to_string());

    let refresh_token = cookies
        .iter()
        .find(|cookie| cookie.name() == REFRESH_TOKEN_COOKIE)
        .map(|cookie| cookie.value().to_string());

    Some(Tokens {
        access_token,
        refresh_token,
    })
}
