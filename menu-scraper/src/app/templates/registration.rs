use askama::Template;

#[derive(Template)]
#[template(path = "registration.html")]
pub struct RegistrationTemplate {
}