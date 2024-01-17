use askama::Template;
use crate::app::view_models::signed_user::SignedUser;

#[derive(Template)]
#[template(path = "nav.html")]
pub struct NavTemplate {
    pub signed_user: Option<SignedUser>,
}