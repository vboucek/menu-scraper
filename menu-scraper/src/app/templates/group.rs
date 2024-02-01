use crate::app::view_models::signed_user::SignedUser;
use askama::Template;
use chrono::NaiveDate;
use db::db::models::{
    Group, GroupCreate, GroupPreview, LunchWithGroup, MenuWithRestaurantAndVotes, UserPreview,
};
use uuid::Uuid;

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

#[derive(Template)]
#[template(path = "group_details.html")]
pub struct GroupDetailsTemplate {
    pub signed_user: Option<SignedUser>,
    pub group: Group,
    pub group_members: Vec<UserPreview>,
    pub group_lunches: Vec<LunchWithGroup>,
}

#[derive(Template)]
#[template(path = "create_lunch_box.html")]
pub struct GroupCreateLunchTemplate {
    pub group_id: Uuid,
    pub min_selection_date: NaiveDate,
}

#[derive(Template)]
#[template(path = "create_lunch_button.html")]
pub struct GroupCreateLunchFormTemplate {
    pub group_id: Uuid,
    pub group_lunches: Vec<LunchWithGroup>,
    pub date: NaiveDate,
}
