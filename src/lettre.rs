use super::email::{Auth, MailerError, Message, Sender};
use async_trait::async_trait;
use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};

pub struct Mailer {
    inner: AsyncSmtpTransport<Tokio1Executor>,
}

impl Mailer {
    pub fn new(relay: &str, auth: Option<Auth>) -> Result<Self, MailerError> {
        let mut mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(relay)?;

        if let Some(auth) = auth {
            let credentials = Credentials::new(auth.username, auth.password);
            mailer = mailer.credentials(credentials);
        }

        Ok(Self {
            inner: mailer.build(),
        })
    }
}

#[async_trait]
impl Sender for Mailer {
    async fn send(&self, message: Message) -> Result<(), MailerError> {
        let mut email = lettre::Message::builder()
            .from(message.from.parse()?)
            .to(message.to.parse()?)
            .subject(message.subject);

        if let Some(rt) = message.reply_to {
            email = email.reply_to(rt.parse()?);
        }

        let msg = match message.html {
            Some(html) => email.multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(message.plain),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(html),
                    ),
            ),
            None => email.body(message.plain),
        };

        self.inner.send(msg?).await?;

        Ok(())
    }
}
