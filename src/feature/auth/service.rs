use crate::feature::auth::password::{generate_hash_password, verify_password_hash_bytes};
use crate::feature::auth::repository::UserRepositoryTrait;
use crate::feature::auth::{entity::UserDB, repository::UserRepository};
use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;
#[cfg_attr(test, automock)]
#[async_trait]
pub trait UserServiceTrait {
    async fn create_user_service(
        &self,
        title: String,
        email: String,
        password: String,
    ) -> Result<UserDB, sqlx::Error>;
    async fn get_user_by_email_service(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<UserDB>, sqlx::Error>;
}
pub struct UserService {
    user_repo: Arc<UserRepository>,
}

impl UserService {
    pub fn new_service(user_repo: Arc<UserRepository>) -> Self {
        Self { user_repo }
    }
}
#[async_trait]
impl UserServiceTrait for UserService {
    async fn create_user_service(
        &self,
        title: String,
        email: String,
        password: String,
    ) -> Result<UserDB, sqlx::Error> {
        let hashed_password = match generate_hash_password(password).await {
            Ok(hash) => hash,
            Err(e) => {
                eprintln!("Failed to hash password: {}", e);
                return Err(sqlx::Error::Configuration("Password hashing failed".into()));
            }
        };
        let password_bytes: Vec<u8> = hashed_password.into_bytes();
        let user = self
            .user_repo
            .create_user(title, email, password_bytes)
            .await?;
        Ok(user)
    }
    async fn get_user_by_email_service(
        &self,
        email: String,
        password: String,
    ) -> Result<Option<UserDB>, sqlx::Error> {
        let user_option = self.user_repo.get_user_by_email(email).await?;

        match user_option {
            Some(user) => {
                if verify_password_hash_bytes(&password, &user.password) {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}
