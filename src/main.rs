mod config;
mod database;
mod email;
mod api;
use tokio_postgres::{connect, Error};
use email::email::NotificationType;

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        let env_vars = config::config::config();
        let (client, connection) = connect(&env_vars.db, tokio_postgres::NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
            eprintln!("Main database connection error: {}", e);
            }
        });
        let conexoes = database::database::fetch_connections(&client).await?;
        let (campaigns_client, campaigns_connection) = connect(&env_vars.campanhas_url, tokio_postgres::NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = campaigns_connection.await {
            eprintln!("Campaigns database connection error: {}", e);
            }
        });


        let restante = database::database::fetch_campaign_left(&campaigns_client).await?;
        println!("Found {} Numbers left in the campaign", restante);
        
        if restante <= 5000 {
            let campanhas_aviso = format!("APENAS {} NÚMEROS RESTANTES NA CAMPANHA!", restante);
            println!("WARNING: Low campaign numbers detected! {}", campanhas_aviso);
                            
            let message = format!("Poucos números na campanha: {}!",restante);
            match api::api::send_whatsapp_message(&env_vars.evo_apikey, &env_vars.evo_url, &env_vars.num_send_to, &message, &conexoes).await {
                Ok(response) => {
                    println!("Whatsapp message sent! Status: {}", response.status());
                },
                Err(e) => {
                    eprintln!("Failed to send whatsapp message! Status: {}", e);
                }
            }
                
            match email::email::send_mail(&env_vars.email, NotificationType::LowCampaignNumbers(restante), &env_vars.gk).await {
                Ok(()) => {
                    println!("Campaign numbers email sent!");
                }
                Err(e) => {
                    eprintln!("Campaign numbers email not sent: {}", e);
                }
            }
        }

        match api::api::get_wallet_balance(&env_vars.api).await {
            Ok(balance) => {
                println!("Current wallet balance: {} {}", balance.message.current_balance, balance.message.currency);
                
                if balance.message.current_balance <= 200.0 {
                    let saldo_aviso = format!("SALDO BAIXO: {}", balance.message.current_balance);
                    println!("WARNING: Low balance detected! {}", saldo_aviso);
                    
                    
                    match email::email::send_mail(
                        &env_vars.email, 
                        NotificationType::LowBalance(balance.message.current_balance), 
                        &env_vars.gk
                    ).await {
                        Ok(()) => {
                            println!("Balance alert email sent!");
                        }
                        Err(e) => {
                            eprintln!("Balance alert email not sent: {}", e);
                        }
                    }

                    let message = format!("Pouco saldo na GUPSHUP: {}!",balance.message.current_balance);
                    match api::api::send_whatsapp_message(&env_vars.evo_apikey, &env_vars.evo_url, &env_vars.num_send_to, &message, &conexoes).await {
                        Ok(response) => {
                            println!("Whatsapp message sent! Status: {}", response.status());
                        },
                        Err(e) => {
                            eprintln!("Failed to send whatsapp message! Status: {}", e);
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to get wallet balance: {}", e);
            }
        }
        
        println!("Sleeping for 60 seconds before next check...");
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
