use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::{Validate, ValidationError};

fn validate_not_nil(id: &Uuid) -> Result<(), ValidationError> {
    if *id == Uuid::nil() {
        Err(ValidationError::new("id_cant_be_nil"))
    } else {
        Ok(())
    }
}
#[derive(Debug, Deserialize, Serialize, Validate, ToSchema)]
pub struct CreateUrlDTO {
    #[validate(url)]
    pub url: String,
}
