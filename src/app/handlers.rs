use crate::app::services::Services;
use crate::feature::auth::handler::UserHandler;
use crate::feature::url::handler::UrlHandler;
use std::sync::Arc;

pub struct Handlers {
    pub url_handler: Arc<UrlHandler>,
    pub user_handle: Arc<UserHandler>,
}
impl Handlers {
    pub fn new(services: Arc<Services>) -> Self {
        Self {
            url_handler: Arc::new(UrlHandler::new_handler(services.url_service.clone())),
            user_handle: Arc::new(UserHandler::new_handler(services.user_service.clone())),
        }
    }
}
