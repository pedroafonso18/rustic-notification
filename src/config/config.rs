use dotenv::dotenv;
use std::env;

pub struct EnvVariables {
    pub db: String,
    pub api: String,
    pub gk: String,
    pub email: String,
    pub campanhas_url: String,
    pub evo_apikey: String,
    pub evo_url: String,
    pub num_send_to: String
}

pub fn config() -> EnvVariables {
    if let Err(e) = dotenv() {
        eprintln!("warning: .env file not loaded: {}", e);
    } 
    let db = env::var("DATABASE_URL").expect("no database_url found in the .env file!");
    let api = env::var("APIKEY").expect("no apikey found in the .env file!");
    let gk = env::var("GOOGLE_KEY").expect("no google_key found in the .env file!");
    let email = env::var("EMAIL").expect("no email found in the .env file!");
    let campanhas_url = env::var("CAMPANHAS_URL").expect("no campanhas_url found in the .env file!");
    let evo_apikey = env::var("EVO_APIKEY").expect("no evo_apikey found in the .env file!");
    let evo_url = env::var("EVO_URL").expect("no evo_url found in the .env file!");
    let num_send_to = env::var("NUM_SEND_TO").expect("no num_send_to found in the .env file!");
    
    EnvVariables {
        db,
        api,
        gk,
        email,
        campanhas_url,
        evo_apikey,
        evo_url,
        num_send_to
    }
}