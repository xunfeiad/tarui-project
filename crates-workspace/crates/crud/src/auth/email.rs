use anyhow::Context;
use error::RtcResult;
use lettre;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use schema::auth::user::AuthUser;
use std::sync::LazyLock;

static SMTP_USERNAME: LazyLock<String> = LazyLock::new(|| {
    std::env::var("smtp_username")
        .context("Get env `stmp_username` failed.")
        .unwrap()
});

static SMTP_PASSWORD: LazyLock<String> = LazyLock::new(|| {
    std::env::var("smtp_password")
        .context("Get env `smtp_password` failed.")
        .unwrap()
});

static EMAIL: LazyLock<String> = LazyLock::new(|| {
    std::env::var("email")
        .context("Get env `email` failed.")
        .unwrap()
});

static SMTP_SERVER: LazyLock<String> = LazyLock::new(|| {
    std::env::var("smtp_server")
        .context("Get env `smtp_server` failed.")
        .unwrap()
});

pub trait SendEmail {
    fn send_email(&self, subject: &str, body: String) -> RtcResult<()>;
}

impl SendEmail for AuthUser {
    fn send_email(&self, subject: &str, body: String) -> RtcResult<()> {
        let email = Message::builder()
            .from(
                EMAIL
                    .parse()
                    .context("Parsed `smtp_username` to mbox type failed.")?,
            )
            .to(self.email.as_ref().unwrap().parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(body)
            .unwrap();
        let creds = Credentials::new(SMTP_USERNAME.to_string(), SMTP_PASSWORD.to_string());

        // Open a remote connection to gmail
        let mailer = SmtpTransport::relay(&SMTP_SERVER)
            .unwrap()
            .credentials(creds)
            .build();
        mailer.send(&email).context("Send email failed")?;
        Ok(())
    }
}
