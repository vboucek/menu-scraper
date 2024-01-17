use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorBannerTemplate {
    pub error: String,
}