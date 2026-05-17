use anyhow::{Context, Result};

use crate::{
    api::models::{
        AgentAuthRequestCodeRequest, AgentAuthRequestCodeResponse, AgentAuthVerifyCodeRequest,
        AgentAuthVerifyCodeResponse,
    },
    cli::{AuthCommand, AuthSubcommand},
    config::{save_api_key, Config},
    http::ApiClient,
    output,
};

pub async fn run(client: &ApiClient, _config: &Config, cmd: AuthCommand, json: bool) -> Result<()> {
    match cmd.command {
        AuthSubcommand::RequestCode { email } => request_code(client, email, json).await,
        AuthSubcommand::Login { email } => login(client, email, json).await,
    }
}

async fn request_code(client: &ApiClient, email: String, json: bool) -> Result<()> {
    let response: AgentAuthRequestCodeResponse = client
        .post(
            "/api/auth/agent/request-code",
            &AgentAuthRequestCodeRequest {
                email: email.clone(),
            },
        )
        .await?;

    output::value_or_json(json, &response, || {
        println!("Sign-in code sent to {email}.");
        Ok(())
    })
}

async fn login(client: &ApiClient, email: String, json: bool) -> Result<()> {
    let _: AgentAuthRequestCodeResponse = client
        .post(
            "/api/auth/agent/request-code",
            &AgentAuthRequestCodeRequest {
                email: email.clone(),
            },
        )
        .await?;
    eprintln!("Check your inbox for a sign-in code from here.now and paste it here.");
    let code = rpassword::prompt_password("Code: ").context("failed to read sign-in code")?;
    let response: AgentAuthVerifyCodeResponse = client
        .post(
            "/api/auth/agent/verify-code",
            &AgentAuthVerifyCodeRequest { email, code },
        )
        .await?;

    save_api_key(&response.api_key)?;

    output::value_or_json(json, &response, || {
        println!("API key saved to ~/.herenow/credentials.");
        Ok(())
    })
}
