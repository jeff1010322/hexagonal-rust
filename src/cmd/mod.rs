use clap::{Parser, Subcommand};
use anyhow::Result;

mod serve;
mod print;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Root {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Serve(serve::Serve),
    Print(print::Print)
}

impl Root {
    pub async fn run() -> Result<()>  {
        match Self::parse().command {
            Commands::Serve(cmd) => cmd.run().await?,
            Commands::Print(cmd) => cmd.run().await?
        }

        Ok(())
    }
}