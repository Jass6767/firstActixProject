use serde_json::json;
use actix_web::{HttpResponse, Responder, get, web, post};
use serde::{Serialize, Deserialize};
use crate::utils::shortner::generate_code;
use crate::models::url;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};


#[derive(Serialize)]
struct ShortenResponse {
    short_url: String,
}

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

#[get("/{code}")]
pub async fn first_page(db: web::Data<DatabaseConnection>, code: web::Path<String>) -> impl Responder {
    let res = url::Entity::find().filter(url::Column::Shorten.eq(code.to_string())).one(db.get_ref()).await;
    match res{
        Ok(Some(record)) => {
            return HttpResponse::Ok().json(json!({"message":format!("{}", record.url)}))
        }
        Ok(None) => {
            return HttpResponse::NotFound().json(
                json!({ "message": "Not Found" })
            );
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(
                json!({ "message": "Internal Server Error" })
            );
        }
    }
    
}

#[post("/shorten")]
pub async fn shorten(db: web::Data<DatabaseConnection> ,data: web::Json<ShortenRequest>) -> impl Responder {
    let code = generate_code(&data.url);
    let res = url::Entity::find().filter(url::Column::Shorten.eq(&code)).one(db.get_ref()).await;
    match res {
        Ok(Some(record)) => {
            return HttpResponse::Ok().json(
                ShortenResponse{
                    short_url: format!("https://localhost/{}", record.shorten)
                }
            );
        }
        Ok(None) => {
            let new_url = url::ActiveModel{
                url: Set(data.url.clone()),
                shorten: Set(code.clone()),
                ..Default::default()
            };
            let result = new_url.insert(db.get_ref()).await;
            match result {
                Ok(_) => {
                    return HttpResponse::Ok().json(
                        ShortenResponse{
                            short_url: format!("https://localhost/{code}")
                        }
                    );
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().json(
                        json!({ "message": "Internal Server Error" })
                    );
                    }
                }
        }
        Err(_) => {
            return HttpResponse::InternalServerError().json(
                json!({ "message": "Internal Server Error" })
            );
        }
    }
}