use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserEdit {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub profile_picture: Option<String>,
}
