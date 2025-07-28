use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Url {
    pub id: Uuid,
    pub alias: String,
    pub url: String,
}
