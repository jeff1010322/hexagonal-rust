use clap::Args;
use anyhow::Result;

/// Arguments for the "serve" command
#[derive(Args, Debug, Clone)]
pub struct Serve {
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}

impl Serve {
    pub async fn run(self) -> Result<()> {
        
        println!("Serve called port: {:?}", &self.port);
        
        Ok(())
    }
}
