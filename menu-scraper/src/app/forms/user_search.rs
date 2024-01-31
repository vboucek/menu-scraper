use anyhow::Error;
use serde::Deserialize;
use crate::app::utils::validation::Validation;

#[derive(Debug, Deserialize)]
pub struct UserSearchQuery {
    pub username: String,
}

impl Validation for UserSearchQuery {
    fn validate(&self) -> Result<(), Error> {
        if self.username.len() > 30 {
            return Err(anyhow::anyhow!("Uživatelské jméno může mít maximálně 30 znaků."));
        }

        Ok(())
    }
}
