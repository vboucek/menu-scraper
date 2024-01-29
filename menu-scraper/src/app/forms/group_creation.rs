use crate::app::utils::validation::Validation;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use anyhow::Error;

/// Multipart form to create a new group
#[derive(MultipartForm, Debug)]
pub struct GroupCreationFormData {
    pub group_name: Text<String>,
    pub group_description: Option<Text<String>>,
    #[multipart(rename = "profile-picture")]
    pub file: Option<TempFile>,
}

impl Validation for GroupCreationFormData {
    fn validate(&self) -> Result<(), Error> {
        if self.group_name.is_empty() {
            return Err(anyhow::anyhow!("Jméno skupiny nemůže být prázdné."));
        }

        if self.group_name.len() > 20 {
            return Err(anyhow::anyhow!(
                "Jméno skupiny může mít maximálně 20 znaků."
            ));
        }

        if let Some(description) = &self.group_description {
            if description.len() > 1000 {
                return Err(anyhow::anyhow!(
                    "Popis skupiny může mít maximálně 1000 znaků."
                ));
            }
        }

        Ok(())
    }
}
