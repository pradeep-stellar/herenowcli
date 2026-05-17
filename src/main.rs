mod api;
mod cli;
mod config;
mod confirm;
mod error;
mod files;
mod http;
mod output;
mod state;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use config::Config;
use http::ApiClient;

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        eprintln!("error: {err:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let cli = Cli::parse();
    let config = Config::from_cli(&cli)?;
    let client = ApiClient::new(config.clone())?;

    match cli.command {
        Command::Auth(cmd) => api::auth::run(&client, &config, cmd, cli.json).await,
        Command::Publish(cmd) => api::sites::run_publish(&client, cmd, cli.json).await,
        Command::Sites(cmd) => api::sites::run_sites(&client, cmd, cli.json).await,
        Command::Drives(cmd) => api::drives::run(&client, cmd, cli.json).await,
        Command::Domains(cmd) => api::domains::run_domains(&client, cmd, cli.json).await,
        Command::Handle(cmd) => api::domains::run_handle(&client, cmd, cli.json).await,
        Command::Links(cmd) => api::domains::run_links(&client, cmd, cli.json).await,
        Command::Variables(cmd) => api::variables::run(&client, cmd, cli.json).await,
        Command::Wallet(cmd) => api::wallet::run(&client, cmd, cli.json).await,
    }
}
