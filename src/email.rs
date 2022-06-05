use async_trait::async_trait;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub reply_to: Option<String>,
    pub cc: Option<String>,
    pub bcc: Option<String>,
    pub subject: String,
    pub plain: String,
    pub html: Option<String>,
}

#[derive(Debug)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

#[async_trait]
pub trait Sender {
    async fn send(&self, message: Message) -> Result<(), MailerError>;
}

#[derive(Error, Debug)]
pub enum MailerError {
    #[error("lettre address error")]
    LettreAddress(#[from] lettre::address::AddressError),
    #[error("lettre error")]
    Lettre(#[from] lettre::error::Error),
    #[error("lettre smtp error")]
    LettreSmtp(#[from] lettre::transport::smtp::Error),
}
