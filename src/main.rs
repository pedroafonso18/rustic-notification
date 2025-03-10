mod config;
mod database;
mod api;
use tokio_postgres::{connect, Error};
use notify_rust::{Notification, Timeout};

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        let (db_url, apikey) = config::config::config();
        let (client, connection) = connect(&db_url, tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let db_ips = database::database::fetch_instances(&client).await?;
        println!("Found {} IPs in database", db_ips.len());

        if db_ips.is_empty() {
            println!("No IPs found, skipping proxy check");
        }
        else {
            for ip in db_ips {
                let ip_aviso = format!("IP INATIVO EM USO: {}", ip);
                println!("{} É IP INATIVO!", ip);
                if let Err(e) = Notification::new()
                    .summary("Serviço de Proxy")
                    .body(&ip_aviso)
                    .timeout(Timeout::Never)
                    .show() {
                    eprintln!("Failed to show notification: {}", e);
                }
            }
        }

        match api::api::get_wallet_balance(&apikey).await {
            Ok(balance) => {
                println!("Current wallet balance: {} {}", balance.message.current_balance, balance.message.currency);
                
                if balance.message.current_balance <= 100.0 {
                    let saldo_aviso = format!("SALDO BAIXO: {}", balance.message.current_balance);
                    println!("WARNING: Low balance detected!");
                    if let Err(e) = Notification::new()
                            .summary("Serviço de Saldo")
                            .body(&saldo_aviso)
                            .timeout(Timeout::Never)
                            .show() {
                            eprintln!("Failed to show notification: {}", e);
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
