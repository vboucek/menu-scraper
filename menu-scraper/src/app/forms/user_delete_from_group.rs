use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct UserDeleteFromGroup {
    #[serde(rename = "user-id")]
    pub user_id: Uuid,
    #[serde(rename = "group-id")]
    pub group_id: Uuid,
}
