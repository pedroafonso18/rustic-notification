use tokio_postgres::{Client, Error};


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

pub async fn fetch_connections(client: &Client) -> Result<Vec<String>, Error> {
    let conexoes = client.query("SELECT name FROM instances WHERE instance_id is not null", &[]).await?;

    let mut conexoes_de_mafia: Vec<String> = Vec::new();
    for conexao in conexoes {
        conexoes_de_mafia.push(
            conexao.get::<_, String>("name")
        );
    }
    Ok(conexoes_de_mafia)
}

pub async fn insert_logs(client: &Client, notif_type: &str) -> Result<(), Error> {
    match client.execute("INSERT INTO logs_notif (notif_type) VALUES ($1)", &[&notif_type]).await {
        Ok(_) => {
            println!("Successfully inserted log!");
            Ok(())
        }
        Err(e) => {
            Err(e)
        }
    }
}

pub async fn fetch_recent_logs(client: &Client, notif_type: &str) -> Result<bool, Error> {
    let rows = client.query(
        "SELECT COUNT(*) FROM logs_notif WHERE notif_type = $1 AND horario >= NOW() - INTERVAL '5 minutes'", 
        &[&notif_type]
    ).await?;

    let count: i64 = rows[0].get(0);
    
    Ok(count > 0)
}