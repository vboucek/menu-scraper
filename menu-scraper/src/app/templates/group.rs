use crate::app::view_models::group::GroupView;
use crate::app::view_models::lunch::MenuWithRestaurantAndVotesView;
use crate::app::view_models::signed_user::SignedUser;
use askama::Template;
use uuid::Uuid;
use chrono::NaiveDate;
use db::db::models::{Group, GroupPreview, Lunch, LunchWithGroup, UserPreview};

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

#[derive(Template)]
#[template(path = "group_details.html")]
pub struct GroupDetailsTemplate {
    pub signed_user: Option<SignedUser>,
    pub group: Group,
    pub group_members: Vec<UserPreview>,
    pub group_lunches: Vec<LunchWithGroup>,
}

#[derive(Template)]
#[template(path = "create_lunch_form.html")]
pub struct GroupCreateLunchTemplate {
    pub group_id: Uuid,
    pub min_selection_date: NaiveDate,
}

#[derive(Template)]
#[template(path = "create_lunch_button.html")]
pub struct GroupCreateLunchFormTemplate {
    pub group_id: Uuid,
    pub lunch: Lunch,
    pub date: NaiveDate,
}

#[derive(Template)]
#[template(path = "group_lunch_menus.html")]
pub struct GroupLunchMenusTemplate {
    pub signed_user: Option<SignedUser>,
    pub lunch: Lunch,
    pub menus: Vec<MenuWithRestaurantAndVotesView>,
}
