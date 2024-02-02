use crate::app::view_models::group::GroupView;
use crate::app::view_models::signed_user::SignedUser;
use askama::Template;
use db::db::models::GroupPreview;
use uuid::Uuid;

#[derive(Template)]
#[template(path = "groups.html")]
pub struct GroupsTemplate {
    pub group_previews: Vec<GroupPreview>,
    pub signed_user: Option<SignedUser>,
}

#[derive(Template)]
#[template(path = "group_creation.html")]
pub struct GroupCreationTemplate {}

#[derive(Template)]
#[template(path = "group_edit.html")]
pub struct GroupEditTemplate {
    pub group: GroupView,
    pub group_id: Uuid,
}
