use askama::Template;
use uuid::Uuid;
use crate::app::view_models::user_preview::UserPreviewView;

#[derive(Template)]
#[template(path = "user_preview_list.html")]
pub struct UserPreviewList {
    pub user_previews: Vec<UserPreviewView>,
    pub group_id: Option<Uuid>,
}
