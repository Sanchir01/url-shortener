use crate::domain::url::Url;
use crate::feature::url::repository::{UrlRepository, UrlRepositoryTrait};
use crate::utils::random::new_random_string;
use async_trait::async_trait;
use mockall::automock;
use std::sync::Arc;
use uuid::Uuid;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait UrlServiceTrait: Send + Sync {
    async fn get_all_url(&self) -> Result<Vec<Url>, sqlx::Error>;
    async fn create_url(&self, url: String, id: Uuid) -> Result<(), sqlx::Error>;
    async fn delete_url(&self, id: Uuid) -> Result<(), sqlx::Error>;
}
#[derive(Clone)]
pub struct UrlService {
    url_repository: Arc<UrlRepository>,
}

impl UrlService {
    pub fn new(url_repository: Arc<UrlRepository>) -> Self {
        Self { url_repository }
    }
}

#[async_trait]
impl UrlServiceTrait for UrlService {
    async fn get_all_url(&self) -> Result<Vec<Url>, sqlx::Error> {
        self.url_repository.get_all_url().await
    }
    async fn create_url(&self, url: String, id: Uuid) -> Result<(), sqlx::Error> {
        let alias = match new_random_string(6).await {
            Ok(a) => a,
            Err(_) => return Err(sqlx::Error::Protocol("random string error".into())),
        };
        self.url_repository.add_url(url, alias, id).await
    }
    async fn delete_url(&self, id: Uuid) -> Result<(), sqlx::Error> {
        self.url_repository.delete_url(id).await
    }
}
