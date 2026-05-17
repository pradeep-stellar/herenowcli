use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "herenow", version, about = "CLI client for the here.now API")]
pub struct Cli {
    #[arg(long, global = true, env = "HERENOW_API_KEY", hide_env_values = true)]
    pub api_key: Option<String>,

    #[arg(long, global = true, default_value = "https://here.now")]
    pub base_url: String,

    #[arg(long, global = true, default_value = "herenow/rust-cli")]
    pub client: String,

    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Auth(AuthCommand),
    Publish(PublishCommand),
    Sites(SitesCommand),
    Drives(DrivesCommand),
    Domains(DomainsCommand),
    Handle(HandleCommand),
    Links(LinksCommand),
    Variables(VariablesCommand),
    Wallet(WalletCommand),
}

#[derive(Debug, Args)]
pub struct AuthCommand {
    #[command(subcommand)]
    pub command: AuthSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum AuthSubcommand {
    RequestCode { email: String },
    Login { email: String },
}

#[derive(Debug, Args)]
pub struct PublishCommand {
    pub path: PathBuf,

    #[arg(long)]
    pub slug: Option<String>,

    #[arg(long)]
    pub claim_token: Option<String>,

    #[arg(long)]
    pub title: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long)]
    pub og_image_path: Option<String>,

    #[arg(long)]
    pub ttl: Option<u64>,

    #[arg(long)]
    pub spa: bool,

    #[arg(long)]
    pub forkable: bool,
}

#[derive(Debug, Args)]
pub struct SitesCommand {
    #[command(subcommand)]
    pub command: SitesSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum SitesSubcommand {
    List,
    Get {
        slug: String,
    },
    Delete {
        slug: String,
        #[arg(long)]
        yes: bool,
    },
    Claim {
        slug: String,
        #[arg(long)]
        claim_token: Option<String>,
    },
    Metadata {
        slug: String,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        og_image_path: Option<String>,
        #[arg(long)]
        ttl: Option<u64>,
        #[arg(long)]
        password: Option<String>,
        #[arg(long)]
        remove_password: bool,
        #[arg(long)]
        spa: Option<bool>,
        #[arg(long)]
        forkable: Option<bool>,
    },
    FromDrive {
        drive_id: String,
        #[arg(long)]
        version: Option<String>,
        #[arg(long)]
        slug: Option<String>,
    },
}

#[derive(Debug, Args)]
pub struct DrivesCommand {
    #[command(subcommand)]
    pub command: DrivesSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DrivesSubcommand {
    List,
    Default,
    Create {
        name: String,
    },
    Get {
        drive_id: String,
    },
    Files {
        drive_id: String,
        #[arg(long)]
        prefix: Option<String>,
    },
    Cat {
        drive_id: String,
        path: String,
    },
    Put {
        drive_id: String,
        path: String,
        #[arg(long)]
        from: PathBuf,
        #[arg(long)]
        if_match: Option<String>,
        #[arg(long)]
        if_none_match: Option<String>,
    },
    DeleteFile {
        drive_id: String,
        path: String,
        #[arg(long)]
        if_match: String,
        #[arg(long)]
        yes: bool,
    },
    Move {
        drive_id: String,
        from: String,
        to: String,
        #[arg(long)]
        if_match: String,
        #[arg(long)]
        overwrite_if_match: Option<String>,
    },
    Tokens {
        drive_id: String,
    },
    Share {
        drive_id: String,
        #[arg(long, default_value = "read")]
        perms: String,
        #[arg(long)]
        prefix: Option<String>,
        #[arg(long)]
        ttl: Option<String>,
        #[arg(long)]
        label: Option<String>,
        #[arg(long)]
        manage_tokens: bool,
    },
}

#[derive(Debug, Args)]
pub struct DomainsCommand {
    #[command(subcommand)]
    pub command: DomainsSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum DomainsSubcommand {
    List,
    Add {
        domain: String,
    },
    Get {
        domain: String,
    },
    Delete {
        domain: String,
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct HandleCommand {
    #[command(subcommand)]
    pub command: HandleSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum HandleSubcommand {
    Get,
    Create {
        handle: String,
    },
    Update {
        handle: String,
    },
    Delete {
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct LinksCommand {
    #[command(subcommand)]
    pub command: LinksSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum LinksSubcommand {
    List,
    Create {
        location: String,
        slug: String,
        #[arg(long)]
        domain: Option<String>,
    },
    Get {
        location: String,
        #[arg(long)]
        domain: Option<String>,
    },
    Update {
        location: String,
        slug: String,
        #[arg(long)]
        domain: Option<String>,
    },
    Delete {
        location: String,
        #[arg(long)]
        domain: Option<String>,
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct VariablesCommand {
    #[command(subcommand)]
    pub command: VariablesSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum VariablesSubcommand {
    List,
    Set {
        name: String,
        #[arg(long)]
        value: Option<String>,
        #[arg(long)]
        allowed_upstream: Vec<String>,
    },
    Delete {
        name: String,
        #[arg(long)]
        yes: bool,
    },
}

#[derive(Debug, Args)]
pub struct WalletCommand {
    #[command(subcommand)]
    pub command: WalletSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum WalletSubcommand {
    Get,
    Set {
        address: String,
    },
    Clear {
        #[arg(long)]
        yes: bool,
    },
}
