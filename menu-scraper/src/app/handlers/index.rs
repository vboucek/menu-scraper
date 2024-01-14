use actix_web::{error::ErrorInternalServerError, get, HttpResponse, Result as ActixResult, web};
use askama::Template;
use db::db::repositories::UserRepository;
use crate::app::templates::index::IndexTemplate;

pub fn index_config(config: &mut web::ServiceConfig) {
    config
        .service(web::resource("").route(web::get().to(index)));
}

async fn index() -> ActixResult<HttpResponse> {
    let template = IndexTemplate { };
    let body = template.render().map_err(ErrorInternalServerError)?;


    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
