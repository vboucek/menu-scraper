use crate::app::view_models::signed_user::SignedUser;
use askama::Template;
use db::db::models::{GroupCreate, GroupPreview};

#[derive(Template)]
#[template(path = "groups.html")]
pub struct GroupsTemplate {
    pub group_previews: Vec<GroupPreview>,
    pub signed_user: Option<SignedUser>,
}

#[derive(Template)]
#[template(path = "group_creation.html")]
pub struct GroupCreationTemplate {
    pub signed_user: Option<SignedUser>,
    pub group: GroupCreate,
}
