use clap::Args;
use anyhow::Result;

/// Arguments for the "set" command
#[derive(Args, Debug, Clone)]
pub struct Set {
    #[arg(long, default_value = "default")]
    pub value: String,
}

impl Set {
    pub async fn run(self) -> Result<()> {
        return Err(anyhow::anyhow!("Set not implemented"));
    }
}
