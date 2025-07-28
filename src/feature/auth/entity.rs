use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate, Serialize, Debug, ToSchema)]
pub struct AuthGoogleDTO {
    #[validate(length(min = 1))]
    pub code: String,
}
#[derive(Deserialize, Validate, Serialize, Debug, ToSchema)]
pub struct RegisterDTO {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub title: String,
    #[validate(length(min = 1))]
    pub password: String,
}
#[derive(Deserialize, Validate, Serialize, Debug, ToSchema)]
pub struct LoginDTO {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub password: String,
}
#[derive(Deserialize, Serialize, Validate, ToSchema, sqlx::FromRow)]
pub struct UserDB {
    pub id: Uuid,
    pub title: String,
    pub email: String,
    pub password: Vec<u8>,
    pub role: UserRole,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub version: i64,
}

#[derive(Deserialize, Serialize, Debug, ToSchema, sqlx::Type)]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum UserRole {
    Admin,
    User,
    Moderator,
}

// Sea Query константы для таблицы users
pub const USERS_TABLE: &str = "users";
pub const USERS_ID: &str = "id";
pub const USERS_TITLE: &str = "title";
pub const USERS_EMAIL: &str = "email";
pub const USERS_PASSWORD: &str = "password";
pub const USERS_ROLE: &str = "role";
pub const USERS_CREATED_AT: &str = "created_at";
pub const USERS_UPDATED_AT: &str = "updated_at";
pub const USERS_VERSION: &str = "version";
