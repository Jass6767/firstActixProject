use std::env;

pub struct Settings {
    pub database_url: String,
    pub server_port: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Set database URL BRO!!!");
        let server_port = env::var("PORT").expect("Set server port BRO!!!");
        Self {
            database_url,
            server_port,
        }
    }
}
