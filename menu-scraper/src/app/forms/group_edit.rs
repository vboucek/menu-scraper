use crate::app::utils::validation::Validation;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text;
use actix_multipart::form::MultipartForm;
use anyhow::Error;

/// Multipart form to edit a group
#[derive(MultipartForm, Debug)]
pub struct GroupEditFormData {
    #[multipart(rename = "group-name")]
    pub group_name: Text<String>,
    #[multipart(rename = "group-description")]
    pub group_description: Text<String>,
    #[multipart(rename = "group-picture")]
    pub file: Option<TempFile>,
}

impl Validation for GroupEditFormData {
    fn validate(&self) -> Result<(), Error> {
        if self.group_name.is_empty() {
            return Err(anyhow::anyhow!("Jméno skupiny nemůže být prázdné."));
        }

        if self.group_name.len() > 50 {
            return Err(anyhow::anyhow!(
                "Jméno skupiny může mít maximálně 50 znaků."
            ));
        }

        if self.group_description.len() > 1000 {
            return Err(anyhow::anyhow!(
                "Popis skupiny může mít maximálně 1000 znaků."
            ));
        }

        Ok(())
    }
}
