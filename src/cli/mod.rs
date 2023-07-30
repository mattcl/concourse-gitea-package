use anyhow::Result;
use clap::{Parser, Subcommand};

mod check;
mod get;
mod out;

#[derive(Debug, Clone, Parser)]
#[command(author, version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

impl Cli {
    pub async fn run() -> Result<()> {
        let cli = Self::parse();
        cli.command.run().await
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    Check(check::Check),
    In(get::Get),
    Out(out::Out),
}

impl Commands {
    pub async fn run(&self) -> Result<()> {
        match self {
            Self::Check(cmd) => cmd.run().await,
            Self::In(cmd) => cmd.run().await,
            Self::Out(cmd) => cmd.run().await,
        }
    }
}
