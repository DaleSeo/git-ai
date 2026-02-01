mod commands;
mod config;
mod git;
mod llm;

use clap::{Parser, Subcommand};
use commands::{commit, config as config_cmd, pr};

#[derive(Parser)]
#[command(name = "git-ai")]
#[command(author, version, about = "AI-powered Git assistant")]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage git-ai configuration
    Config(config_cmd::ConfigArgs),

    /// Generate AI-powered commit message
    Commit(commit::CommitArgs),

    /// Generate PR title and description
    Pr(pr::PrArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config(args) => config_cmd::run(args).await?,
        Commands::Commit(args) => commit::run(args).await?,
        Commands::Pr(args) => pr::run(args).await?,
    }

    Ok(())
}
