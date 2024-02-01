use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserAddInGroupForm {
    pub id: Uuid,
    pub username: String,
    #[serde(rename = "profile-picture")]
    pub profile_picture: String,
    // Option group id to determine between group create and group edit (we dont have group id in create)
    #[serde(rename = "group-id")]
    pub group_id: Option<Uuid>,
}