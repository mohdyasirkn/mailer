use utils::recepients::{self, mail_list_from_csv};

mod utils;
#[tokio::main]
async fn main() {
    let sender_email = std::env::args()
        .nth(1)
        .expect("Please give your email id bruw");
    let sender_passwd = std::env::args().nth(2).expect("also your password");
    dbg!(&sender_email);

    let from = &sender_email.clone();

    let smtp_credentials =
        lettre::transport::smtp::authentication::Credentials::new(sender_email, sender_passwd);

    let smtp_provider = "smtp.zoho.in";

    let mailer =
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::starttls_relay(smtp_provider)
            .unwrap()
            .credentials(smtp_credentials)
            .port(587)
            .build();

    let maillist = mail_list_from_csv();

    for recepient in maillist.iter() {
        println!(
            "Sending mail to name:{}, mailid:{}, team: {}",
            recepient.name, recepient.mailid, recepient.team
        );
        utils::mail::send_email_smtp(
            &mailer,
            from.as_str(),
            &recepient.mailid,
            utils::mail_content::subject().as_str(),
            utils::mail_content::mail_content(recepient),
        )
        .await
        .unwrap();
    }

    // println!(
    //     "Successfully sent email(s) to {}",
    //     utils::recepients::mail_list()
    // );
}
