use askama::Template;
use crate::app::view_models::user_preview::UserPreviewView;

#[derive(Template)]
#[template(path = "user_group_preview.html")]
pub struct UserGroupPreview {
    pub user_preview: UserPreviewView,
}