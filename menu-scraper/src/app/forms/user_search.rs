use crate::app::utils::validation::Validation;
use anyhow::Error;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserSearchQuery {
    pub username: String,
    #[serde(rename = "group-id")]
    pub group_id: Option<Uuid>,
}

impl Validation for UserSearchQuery {
    fn validate(&self) -> Result<(), Error> {
        if self.username.len() > 30 {
            return Err(anyhow::anyhow!(
                "Uživatelské jméno může mít maximálně 30 znaků."
            ));
        }

        Ok(())
    }
}
