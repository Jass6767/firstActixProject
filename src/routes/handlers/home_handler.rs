use serde_json::json;
use actix_web::{HttpResponse, Responder, get, web, post};
use serde::{Serialize, Deserialize};
use crate::utils::shortner::generate_code;


#[derive(Serialize)]
struct ShortenResponse {
    short_code: String,
}

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[get("/")]
pub async fn first_page() -> impl Responder {
    let res = json!({ "message": format!("URL Shortener Running") });
    HttpResponse::Ok().json(res)
}

#[post("/shorten")]
pub async fn shorten(data: web::Json<ShortenRequest>) -> impl Responder {
    let code = generate_code(&data.url);
    HttpResponse::Ok().json(
        ShortenResponse{
            short_code: code
        }
    )
}