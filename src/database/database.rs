use tokio_postgres::{Client, Error};

pub async fn fetch_instances(client: &Client) -> Result<Vec<String>, Error> {
    let rows = client.query("SELECT proxy FROM instances WHERE proxy IS NOT NULL AND proxy NOT in (SELECT ip FROM proxy)", &[]).await?;

    let mut proxyvec: Vec<String> = Vec::new();
    for row in rows {
        let ip: &str = row.get("proxy");
        proxyvec.push(ip.to_string());
    }
    Ok(proxyvec)
}

pub async fn fetch_campaign_left(client: &Client) -> Result<i64, Error> {
    let campanha_rows = client.query("SELECT nome_campanha FROM gerenciamento_campanhas WHERE ativa = TRUE LIMIT 1", &[]).await?;
    
    if campanha_rows.is_empty() {
        return Ok(0);
    }
    
    let nome_campanha: &str = campanha_rows[0].get("nome_campanha");
    
    let contagem_rows = client.query("SELECT COUNT(*) FROM campanhas WHERE campanha = $1 AND disparado = FALSE", &[&nome_campanha]).await?;
    
    let contagem: i64 = contagem_rows[0].get(0);
    
    Ok(contagem)
}