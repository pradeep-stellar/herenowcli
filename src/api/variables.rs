use anyhow::{Context, Result};
use serde_json::Value;

use crate::{
    api::models::VariableSetRequest,
    cli::{VariablesCommand, VariablesSubcommand},
    confirm,
    http::ApiClient,
    output,
};

pub async fn run(client: &ApiClient, cmd: VariablesCommand, json_output: bool) -> Result<()> {
    match cmd.command {
        VariablesSubcommand::List => {
            output::json(&client.get::<Value>("/api/v1/me/variables").await?)
        }
        VariablesSubcommand::Set {
            name,
            value,
            allowed_upstream,
        } => {
            let value = match value {
                Some(value) => value,
                None => rpassword::prompt_password("Value: ")
                    .context("failed to read variable value")?,
            };
            let response: Value = client
                .put(
                    &format!("/api/v1/me/variables/{name}"),
                    &VariableSetRequest {
                        value,
                        allowed_upstreams: allowed_upstream,
                    },
                )
                .await?;
            output::value_or_json(json_output, &response, || {
                println!("Variable {name} saved.");
                Ok(())
            })
        }
        VariablesSubcommand::Delete { name, yes } => {
            confirm::confirm_or_bail(&format!("Delete variable '{name}'?"), yes)?;
            let response: Value = client
                .delete(&format!("/api/v1/me/variables/{name}"))
                .await?;
            output::json_or_done(json_output, &response, "Variable deleted.")
        }
    }
}
