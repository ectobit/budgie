use super::email::{MailerError, Message, Sender};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use http::StatusCode;
use serde::Serialize;
use serde_json::{json, Value};
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

pub type DynMailer = Arc<dyn Sender + Send + Sync>;

pub struct Server {
    port: u16,
    mailer: DynMailer,
}

impl Server {
    pub fn new(port: u16, mailer: DynMailer) -> Self {
        Self { port, mailer }
    }

    pub async fn serve(self) -> Result<(), ServerError> {
        let app = Router::new()
            .route("/", get(health))
            .route("/send", post(send))
            .layer(TraceLayer::new_for_http())
            .layer(Extension(self.mailer));

        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        info!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

async fn send(
    Json(message): Json<Message>,
    Extension(mailer): Extension<DynMailer>,
) -> impl IntoResponse {
    match mailer.send(message).await {
        Ok(_) => (StatusCode::CREATED, Json(Response { error: None })).into_response(),
        Err(err) => {
            error!(error = ?err);
            err.into_response()
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

impl IntoResponse for MailerError {
    fn into_response(self) -> response::Response {
        let status = match self {
            MailerError::LettreAddress(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (
            status,
            Json(Response {
                error: Some(self.to_string()),
            }),
        )
            .into_response()
    }
}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("hyper error")]
    Hyper(#[from] hyper::Error),
}
