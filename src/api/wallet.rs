use anyhow::Result;
use serde_json::Value;

use crate::{
    api::models::WalletPatchRequest,
    cli::{WalletCommand, WalletSubcommand},
    confirm,
    http::ApiClient,
    output,
};

pub async fn run(client: &ApiClient, cmd: WalletCommand, json_output: bool) -> Result<()> {
    match cmd.command {
        WalletSubcommand::Get => output::json(&client.get::<Value>("/api/v1/wallet").await?),
        WalletSubcommand::Set { address } => {
            let response: Value = client
                .patch(
                    "/api/v1/wallet",
                    &WalletPatchRequest {
                        address: Some(address),
                    },
                )
                .await?;
            output::json_or_done(json_output, &response, "Wallet saved.")
        }
        WalletSubcommand::Clear { yes } => {
            confirm::confirm_or_bail("Clear wallet address?", yes)?;
            let response: Value = client
                .patch("/api/v1/wallet", &WalletPatchRequest { address: None })
                .await?;
            output::json_or_done(json_output, &response, "Wallet cleared.")
        }
    }
}
