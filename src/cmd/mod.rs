use clap::{Parser, Subcommand};
use anyhow::Result;

mod serve;
mod set;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Root {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Serve(serve::Serve),
    Set(set::Set)
}

impl Root {
    pub async fn run() -> Result<()>  {
        match Self::parse().command {
            Commands::Serve(cmd) => cmd.run().await?,
            Commands::Set(cmd) => cmd.run().await?
        }

        Ok(())
    }
}