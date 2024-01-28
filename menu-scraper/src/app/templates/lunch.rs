use crate::app::view_models::lunch::LunchPreviewView;
use askama::Template;

#[derive(Template)]
#[template(path = "lunch_preview_list.html")]
pub struct LunchPreviewListTemplate {
    pub lunches: Vec<LunchPreviewView>,
}
