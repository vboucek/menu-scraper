use uuid::Uuid;
use crate::app::view_models::user_preview::UserPreviewView;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupView {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub picture: Option<String>,
    pub users: Vec<UserPreviewView>
}
