use crate::app::view_models::user_edit::UserEdit;
use askama::Template;

#[derive(Template)]
#[template(path = "user_edit.html")]
pub struct UserEditTemplate {
    pub user: UserEdit,
}
