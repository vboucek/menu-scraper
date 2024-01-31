use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserAddInGroupForm {
    pub id: Uuid,
    pub username: String,
    #[serde(rename = "profile-picture")]
    pub profile_picture: String,
}