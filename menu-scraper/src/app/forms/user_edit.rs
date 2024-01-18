use crate::app::utils::validation::Validation;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use anyhow::Error;

/// Multipart form to edit user
#[derive(MultipartForm, Debug)]
pub struct UserEditFormData {
    pub username: Text<String>,
    #[multipart(rename = "old-password")]
    pub old_password: Text<String>,
    #[multipart(rename = "new-password")]
    pub new_password: Text<String>,
    pub email: Text<String>,
    #[multipart(rename = "profile-picture")]
    pub file: Option<TempFile>,
}

impl Validation for UserEditFormData {
    fn validate(&self) -> Result<(), Error> {
        if self.username.is_empty() {
            return Err(anyhow::anyhow!("Uživatelské jméno nemůže být prázdné."));
        }

        if self.username.len() > 30 {
            return Err(anyhow::anyhow!(
                "Uživatelské jméno může mít maximálně 30 znaků."
            ));
        }

        self.is_valid_email(&self.email)?;

        if self.email.len() > 100 {
            return Err(anyhow::anyhow!("Email může mít maximálně 100 znaků."));
        }

        if self.old_password.len() < 12 {
            return Err(anyhow::anyhow!("Heslo musí mít alespoň 12 znaků."));
        }

        if self.old_password.len() > 100 {
            return Err(anyhow::anyhow!("Heslo může mít maximálně 100 znaků."));
        }

        if self.new_password.0 != "" && self.new_password.0.len() < 12 {
            return Err(anyhow::anyhow!("Heslo musí mít alespoň 12 znaků."));
        }

        if self.new_password.0 != "" && self.new_password.0.len() < 12 {
            return Err(anyhow::anyhow!("Heslo může mít maximálně 100 znaků."));
        }

        Ok(())
    }
}
