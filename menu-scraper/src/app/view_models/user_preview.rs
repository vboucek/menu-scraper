use db::db::models::UserPreview;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserPreviewView {
    pub id: Uuid,
    pub username: String,
    pub profile_picture: Option<String>,
}

impl From<UserPreview> for UserPreviewView {
    fn from(user_preview: UserPreview) -> Self {
        UserPreviewView {
            id: user_preview.id,
            username: user_preview.username,
            profile_picture: user_preview.profile_picture,
        }
    }
}
