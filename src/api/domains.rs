use anyhow::Result;
use serde_json::Value;

use crate::{
    api::models::{CreateDomainRequest, HandleRequest, LinkRequest},
    cli::{
        DomainsCommand, DomainsSubcommand, HandleCommand, HandleSubcommand, LinksCommand,
        LinksSubcommand,
    },
    confirm,
    http::ApiClient,
    output,
};

pub async fn run_domains(client: &ApiClient, cmd: DomainsCommand, json_output: bool) -> Result<()> {
    match cmd.command {
        DomainsSubcommand::List => output::json(&client.get::<Value>("/api/v1/domains").await?),
        DomainsSubcommand::Add { domain } => {
            let response: Value = client
                .post("/api/v1/domains", &CreateDomainRequest { domain })
                .await?;
            output::json(&response)
        }
        DomainsSubcommand::Get { domain } => output::json(
            &client
                .get::<Value>(&format!("/api/v1/domains/{domain}"))
                .await?,
        ),
        DomainsSubcommand::Delete { domain, yes } => {
            confirm::confirm_or_bail(&format!("Delete domain '{domain}'?"), yes)?;
            let response: Value = client.delete(&format!("/api/v1/domains/{domain}")).await?;
            output::json_or_done(json_output, &response, "Domain deleted.")
        }
    }
}

pub async fn run_handle(client: &ApiClient, cmd: HandleCommand, json_output: bool) -> Result<()> {
    match cmd.command {
        HandleSubcommand::Get => output::json(&client.get::<Value>("/api/v1/handle").await?),
        HandleSubcommand::Create { handle } => output::json(
            &client
                .post::<_, Value>("/api/v1/handle", &HandleRequest { handle })
                .await?,
        ),
        HandleSubcommand::Update { handle } => output::json(
            &client
                .patch::<_, Value>("/api/v1/handle", &HandleRequest { handle })
                .await?,
        ),
        HandleSubcommand::Delete { yes } => {
            confirm::confirm_or_bail("Delete your handle?", yes)?;
            let response: Value = client.delete("/api/v1/handle").await?;
            output::json_or_done(json_output, &response, "Handle deleted.")
        }
    }
}

pub async fn run_links(client: &ApiClient, cmd: LinksCommand, json_output: bool) -> Result<()> {
    match cmd.command {
        LinksSubcommand::List => output::json(&client.get::<Value>("/api/v1/links").await?),
        LinksSubcommand::Create {
            location,
            slug,
            domain,
        } => output::json(
            &client
                .post::<_, Value>(
                    "/api/v1/links",
                    &LinkRequest {
                        location,
                        slug,
                        domain,
                    },
                )
                .await?,
        ),
        LinksSubcommand::Get { location, domain } => {
            let path = link_path(&location, domain.as_deref());
            output::json(&client.get::<Value>(&path).await?)
        }
        LinksSubcommand::Update {
            location,
            slug,
            domain,
        } => {
            let path = link_path(&location, domain.as_deref());
            output::json(
                &client
                    .patch::<_, Value>(
                        &path,
                        &LinkRequest {
                            location,
                            slug,
                            domain,
                        },
                    )
                    .await?,
            )
        }
        LinksSubcommand::Delete {
            location,
            domain,
            yes,
        } => {
            let label = match &domain {
                Some(domain) => format!("Delete link '{location}' on domain '{domain}'?"),
                None => format!("Delete handle link '{location}'?"),
            };
            confirm::confirm_or_bail(&label, yes)?;
            let path = link_path(&location, domain.as_deref());
            let response: Value = client.delete(&path).await?;
            output::json_or_done(json_output, &response, "Link deleted.")
        }
    }
}

fn link_path(location: &str, domain: Option<&str>) -> String {
    let encoded = if location.is_empty() {
        "__root__".to_string()
    } else {
        urlencoding::encode(location).to_string()
    };
    match domain {
        Some(domain) => format!(
            "/api/v1/links/{encoded}?domain={}",
            urlencoding::encode(domain)
        ),
        None => format!("/api/v1/links/{encoded}"),
    }
}
