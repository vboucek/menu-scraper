use crate::app::utils::validation::Validation;
use anyhow::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoginFormData {
    pub email: String,
    pub password: String,
}

impl Validation for LoginFormData {
    fn validate(&self) -> Result<(), Error> {
        self.is_valid_email(&self.email)?;

        if self.email.len() > 50 {
            return Err(anyhow::anyhow!("Email může mít maximálně 100 znaků."));
        }

        if self.password.len() < 12 {
            return Err(anyhow::anyhow!("Heslo musí mít alespoň 12 znaků."));
        }

        if self.password.len() > 100 {
            return Err(anyhow::anyhow!("Heslo může mít maximálně 12 znaků."));
        }

        Ok(())
    }
}
