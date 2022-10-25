//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

use anyhow::{Context, Result};
use budgie::{
    email::Auth,
    lettre::Mailer,
    server::{DynMailer, Server},
};
use clap::Parser;
use std::{
    fmt, process,
    str::{self, FromStr},
    sync::Arc,
};
use thiserror::Error;
use tracing::info;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Cli {
    #[clap(long, env, value_parser = valid_port, default_value_t = 3000)]
    port: u16,

    #[clap(long, env,  value_parser, default_value_t = String::from("smtp.ectobit.com"))]
    smtp_relay: String,

    #[clap(long, env, value_parser)]
    smtp_username: Option<String>,

    #[clap(long, env, value_parser)]
    smtp_password: Option<String>,

    #[clap(env, value_parser, default_value_t = LogFormat::Plain)]
    log_format: LogFormat,
}

fn main() {
    process::exit(match run() {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {err:?}");
            1
        }
    });
}

#[tokio::main]
async fn run() -> Result<()> {
    let cli = Cli::parse();

    let filter = tracing_subscriber::EnvFilter::from_default_env();
    let builder = tracing_subscriber::fmt().with_env_filter(filter);

    match cli.log_format {
        LogFormat::Plain => builder.init(),
        LogFormat::Json => builder.json().init(),
    }

    let mut auth: Option<Auth> = None;
    if cli.smtp_username.is_some() && cli.smtp_username.is_some() {
        info!("smtp auth configured");
        auth = Some(Auth {
            username: cli.smtp_username.unwrap(),
            password: cli.smtp_password.unwrap(),
        })
    }

    let mailer = Mailer::new(&cli.smtp_relay, auth).context("failed creating mailer")?;

    let mailer = Arc::new(mailer) as DynMailer;

    let server = Server::new(cli.port, mailer);
    server.serve().await.context("failed running server")?;

    Ok(())
}

fn valid_port(s: &str) -> Result<u16, String> {
    if let Ok(port) = u16::from_str(s) {
        if (1..=65535).contains(&port) {
            return Ok(port);
        }
    }

    Err(format!("Invalid port: {s}"))
}

#[derive(Debug, Clone)]
enum LogFormat {
    Plain,
    Json,
}

impl fmt::Display for LogFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let v = match self {
            LogFormat::Plain => "plain",
            LogFormat::Json => "json",
        };
        write!(f, "{v}")
    }
}

impl str::FromStr for LogFormat {
    type Err = LogFormatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "plain" => Ok(LogFormat::Plain),
            "json" => Ok(LogFormat::Json),
            _ => Err(LogFormatError::Invalid),
        }
    }
}

#[non_exhaustive]
#[derive(Debug, Error)]
enum LogFormatError {
    #[error("invalid log format")]
    Invalid,
}
