use crate::app::repositories::Repositories;
use crate::feature::auth::service::UserService;
use crate::feature::url::service::UrlService;
use std::sync::Arc;

#[derive(Clone)]
pub struct Services {
    pub url_service: Arc<UrlService>,
    pub user_service: Arc<UserService>,
}

impl Services {
    pub fn new(repo: Arc<Repositories>) -> Self {
        Self {
            url_service: Arc::new(UrlService::new(repo.url_repository.clone())),
            user_service: Arc::new(UserService::new_service(repo.user_repository.clone())),
        }
    }
}
