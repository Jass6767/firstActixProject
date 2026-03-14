mod utils;
mod routes;
mod models;
mod config;
use config::database::connect;
use config::settings::Settings;
use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web, http};
use env_logger;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG","actix_web=info");
        }
    }
    
    dotenv::dotenv().ok();
    env_logger::init();
    let settings = Settings::new();
    let db = connect(&settings.database_url).await;
    let db = web::Data::new(db);

    let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://url-shortner-rouge-beta.vercel.app/")
            .allowed_methods(vec!["POST", "GET"])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(db.clone())
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}