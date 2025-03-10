use dotenv::dotenv;
use std::env;

pub fn config() -> (String, String) {
    if let Err(e) = dotenv() {
        eprintln!("warning: .env file not loaded: {}", e);
    } 
    let db = env::var("DATABASE_URL").expect("no database_url found in the .env file!");
    let api = env::var("APIKEY").expect("no apikey found in the .env file!");
    return(db, api);
}