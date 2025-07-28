use axum_extra::extract::cookie::CookieJar;
use chrono::{Duration, Utc};
use cookie::{Cookie, SameSite};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tower_http::follow_redirect::ResponseFuture;
use uuid::Uuid;

use crate::{
    feature::auth::entity::UserRole,
    utils::constants::{ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE},
};

const TOKEN_EXPIRATION_HOURS: i64 = 24;
const TOKEN_EXPIRATION_MINUTES: i64 = 15;
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: Uuid,
    pub sub: String,
    pub role: UserRole,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
}
pub async fn get_jwt(id: Uuid, role: UserRole, exp: i64) -> Result<String, String> {
    let token = encode(
        &Header::default(),
        &Claims {
            id,
            sub: id.to_string(),
            role,
            exp: (Utc::now() + Duration::hours(exp)).timestamp() as usize,
            iat: Utc::now().timestamp() as usize,
            jti: Uuid::new_v4().to_string(),
        },
        &EncodingKey::from_secret("JWT_KEY".as_bytes()),
    )
    .unwrap();
    Ok(token)
}

pub async fn get_two_jwt(id: Uuid, role: UserRole) -> Result<(String, String), String> {
    let refresh_token = get_jwt(id, role.clone(), TOKEN_EXPIRATION_HOURS).await?;
    let access_token = get_jwt(id, role, TOKEN_EXPIRATION_MINUTES).await?;
    Ok((refresh_token, access_token))
}
pub async fn set_jwt(id: Uuid, role: UserRole) -> Result<CookieJar, String> {
    let (refresh_token, access_token) = get_two_jwt(id, role).await?;

    let mut jar = CookieJar::new();

    let refresh_cookie = Cookie::build((REFRESH_TOKEN_COOKIE, refresh_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(cookie::time::Duration::hours(TOKEN_EXPIRATION_HOURS as i64))
        .path("/")
        .build();

    let access_cookie = Cookie::build((ACCESS_TOKEN_COOKIE, access_token))
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .max_age(cookie::time::Duration::minutes(
            TOKEN_EXPIRATION_MINUTES as i64,
        ))
        .path("/")
        .build();

    jar = jar.add(refresh_cookie);
    jar = jar.add(access_cookie);

    Ok(jar)
}

pub async fn decode_jwt(token: &str) -> Result<Claims, String> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret("JWT_KEY".as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| e.to_string())?;

    Ok(token_data.claims)
}
