use serde_json::json;
use actix_web::{HttpResponse, Responder, get, web};




#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    let res = json!({ "message": format!("Hello, {name}") });
    HttpResponse::Ok().json(res)
}

#[get("/test/{token}")]
pub async fn test(token: web::Path<String>) -> impl Responder {
    match token.as_str() {
        "12" => HttpResponse::Ok().json(json!({ "message": "Test 12" })),
        _ => HttpResponse::ImATeapot().json(json!({ "message": "i am teapot" })),
    }
}