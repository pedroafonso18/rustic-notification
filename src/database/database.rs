use tokio_postgres::{Client, Error};

pub async fn fetch_instances(client: &Client) -> Result<Vec<String>, Error> {
    let rows = client.query("SELECT proxy FROM instances WHERE proxy NOT in (SELECT ip FROM proxy)", &[]).await?;

    let mut proxyvec: Vec<String> = Vec::new();
    for row in rows {
        let ip: &str = row.get("proxy");
        proxyvec.push(ip.to_string());
    }
    Ok(proxyvec)
}
