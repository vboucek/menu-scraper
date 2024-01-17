use askama::Template;

#[derive(Template)]
#[template(path = "nav.html")]
pub struct NavTemplate {
}