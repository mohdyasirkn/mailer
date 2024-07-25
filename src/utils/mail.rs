// Email sending function
use lettre::{
    message::{header, MultiPart, SinglePart},
    Message, Transport,
};
use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport,
    Tokio1Executor,
};

pub async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .singlepart(
            SinglePart::builder()
                .header(header::ContentType::TEXT_HTML)
                .body(String::from(body)),
        )
        .expect("failed to build email");
    mailer.send(email).await?;

    Ok(())
}
