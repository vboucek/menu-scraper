use crate::app::view_models::user_preview::UserPreviewView;
use askama::Template;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "user_preview_list.html")]
pub struct UserPreviewList {
    pub user_previews: Vec<UserPreviewView>,
    pub group_id: Option<Uuid>,
}
