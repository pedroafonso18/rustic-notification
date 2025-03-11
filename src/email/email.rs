use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::error::Error;


pub enum NotificationType {
    LowBalance(f64),   
    LowCampaignNumbers(i64),  
}

pub async fn send_mail(email_to: &str, notification_type: NotificationType, app_password: &str) -> Result<(), Box<dyn Error>> {
    let (subject, body) = match notification_type {
        NotificationType::LowBalance(balance) => (
            "LOW GUPSHUP BALANCE ALERT",
            format!("Balanço total da GUPSHUP: {}. Favor recarregar o saldo.", balance)
        ),
        NotificationType::LowCampaignNumbers(remaining) => (
            "LOW CAMPAIGN NUMBERS ALERT",
            format!("Apenas {} números restantes na campanha ativa! Favor adicionar mais números.", remaining)
        ),
    };

    let email = Message::builder()
        .from(email_to.parse()?)
        .to(email_to.parse()?)
        .subject(subject)
        .body(body)?;

    let gk = app_password.replace("_", " ");
    let creds = Credentials::new(
        email_to.to_string(),
        gk
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            println!("Email sent successfully: {}", subject);
            Ok(())
        },
        Err(e) => {
            println!("Could not send email: {}", e);
            Err(Box::new(e))
        },
    }
}