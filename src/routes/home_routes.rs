use actix_web::web;
use crate::routes::handlers::home_handler;

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/home"))
        .service(home_handler::shorten)
        .service(home_handler::first_page);
}