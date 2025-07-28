use crate::feature::auth::repository::UserRepository;
use crate::feature::url::repository::UrlRepository;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

#[derive(Clone)]
pub struct Repositories {
    pub url_repository: Arc<UrlRepository>,
    pub user_repository: Arc<UserRepository>,
}

impl Repositories {
    pub fn new(pg: Pool<Postgres>) -> Self {
        Self {
            url_repository: Arc::new(UrlRepository::new_url_repository(pg.clone())),
            user_repository: Arc::new(UserRepository::new_user_repository(pg.clone())),
        }
    }
}
