use std::env;

pub struct Settings {
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("Set database URL BRO!!!");
        Self {
            database_url,
        }
    }
}
