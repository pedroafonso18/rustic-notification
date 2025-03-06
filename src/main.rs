mod config;
mod database;
use tokio_postgres::{connect, Error};
use notify_rust::{Notification, Timeout};

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        let db_url = config::config::config();
        let (client, connection) = connect(&db_url, tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let db_ips = database::database::fetch_instances(&client).await?;
        println!("Found {} IPs in database", db_ips.len());

        for ip in db_ips {
            let proxy = database::database::fetch_innactive_proxies(&client, &ip).await?;
            if proxy == "" {
                let ip_aviso = format!("IP INATIVO EM USO: {}", ip);
                println!("{} É IP INATIVO!", ip);
                Notification::new()
                    .summary("Serviço de Proxy")
                    .body(&ip_aviso)
                    .timeout(Timeout::Never)
                    .show().unwrap();
            } 
        }
        
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
