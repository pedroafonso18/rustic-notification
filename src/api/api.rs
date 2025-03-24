use reqwest;
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    RequestError(reqwest::Error),
    JsonParseError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::RequestError(e) => write!(f, "Request error: {}", e),
            ApiError::JsonParseError(e) => write!(f, "JSON parse error: {}", e),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::RequestError(error)
    }
}

#[derive(Serialize)]
struct WhatsappBodyRequest {
    number: String,
    text: String, 
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Message {
    #[serde(rename = "accountLimit")]
    pub account_limit: f64,
    
    #[serde(rename = "currentBalance")]
    pub current_balance: f64,
    
    #[serde(rename = "overDraftLimit")]
    pub over_draft_limit: f64,
    
    #[serde(rename = "validUpto")]
    pub valid_upto: u64,
    
    #[serde(rename = "createdOn")]
    pub created_on: u64,
    
    pub currency: String,
    
    #[serde(rename = "alertBalance")]
    pub alert_balance: f64,
    
    #[serde(rename = "metaData")]
    pub meta_data: String,
    
    pub source: String,
    
    #[serde(rename = "walletId")]
    pub wallet_id: u32,
    
    #[serde(rename = "walletType")]
    pub wallet_type: String,
    
    #[serde(rename = "skipDeduction")]
    pub skip_deduction: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ApiResponse {
    pub status: String,
    pub message: Message,
}

pub async fn get_wallet_balance(apikey: &str) -> Result<ApiResponse, ApiError> {
    let client = reqwest::Client::new();
    
    let auth_response = client
        .get("https://api.gupshup.io/sm/api/auth/ticket?moduleId=6&")
        .header("apikey", apikey)
        .send()
        .await?;
        
    let token = auth_response.text().await?;
    
    let api_preview = if token.len() > 5 {
        format!("{}...", &token[0..5])
    } else {
        "Invalid API key".to_string()
    };
    println!("Fetching balance with API key: {}", api_preview);
    
    let response = client
        .get("https://webwallet.gupshup.io/WebWallet/wallet/WHATSAPP/balance?precision=4")
        .header("authorization", token)
        .send()
        .await?;
    
    println!("API Response status: {}", response.status());
    
    let body_text = response.text().await?;    
    match serde_json::from_str::<ApiResponse>(&body_text) {
        Ok(balance) => Ok(balance),
        Err(e) => {
            println!("JSON parsing error: {}", e);
            Err(ApiError::JsonParseError(e.to_string()))
        }
    }
}


pub async fn send_whatsapp_message(evo_apikey: &str, evo_url: &str, num_send_to: &str, message: &str) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();

    let payload = WhatsappBodyRequest {
        number: num_send_to.to_string(),
        text: message.to_string(),
    };

    let response = client
        .post(evo_url)
        .header("apikey", evo_apikey)
        .json(&payload)
        .send()
        .await?;
    
    Ok(response)
}