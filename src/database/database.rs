use tokio_postgres::{Client, Error};

pub async fn fetch_instances(client: &Client) -> Result<Vec<String>, Error> {
    let rows = client.query("SELECT proxy FROM instances WHERE proxy IS NOT null", &[]).await?;

    let mut proxyvec: Vec<String> = Vec::new();
    for row in rows {
        let ip: &str = row.get("proxy");
        proxyvec.push(ip.to_string());
    }
    Ok(proxyvec)
}

pub async fn fetch_innactive_proxies(client: &Client, proxy: &str) -> Result<String, Error> {
    let rows = client.query("SELECT ip FROM proxy WHERE ip = $1", &[&proxy]).await?;
    if rows.is_empty() {
        Ok(String::new())
    } else {
        let ip: &str = rows[0].get("ip");
        Ok(ip.to_string())
    }
}