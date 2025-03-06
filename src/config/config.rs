use dotenv::dotenv;
use std::env;

pub fn config() -> String {
    if let Err(e) = dotenv() {
        eprintln!("warning: .env file not loaded: {}", e);
    } 
    env::var("DATABASE_URL").expect("no database_url found in the .env file!")
}