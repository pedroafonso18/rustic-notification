use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use std::error::Error;

pub async fn send_mail(email_to: &str, balance: f64, app_password: &str) -> Result<(), Box<dyn Error>> {
    let email = Message::builder()
        .from(email_to.parse()?)
        .to(email_to.parse()?)
        .subject("LOW BALANCE")
        .body(format!("Balanço total da GUPSHUP: {}", balance))?;

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
            println!("Email sent successfully!");
            Ok(())
        },
        Err(e) => {
            println!("Could not send email: {}", e);
            Err(Box::new(e))
        },
    }
}