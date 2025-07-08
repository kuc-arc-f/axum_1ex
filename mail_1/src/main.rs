use lettre::{
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
    message::{Mailbox, SinglePart, header::ContentType},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = "test@gmail.com";
    let app_password = "";  // Google で発行されたもの
    let send_to = "test@gmail.com";

    let email = Message::builder()
        .from(Mailbox::new(None, username.parse()?))
        .to(Mailbox::new(None, send_to.parse()?))
        .subject("Gmail送信テスト 0708-PM-2")
        .singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_PLAIN)
                .body("こんにちは from Rust!".to_string())
        )?;

    let creds = Credentials::new(username.to_string(), app_password.to_string());
    let mailer = SmtpTransport::starttls_relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;
    println!("Gmail送信成功！");
    Ok(())
}
