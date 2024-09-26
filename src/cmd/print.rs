use clap::Args;
use anyhow::Result;

use crate::core::services::printer::PrinterService;
use crate::core::adapters::printer_mock::PrinterMockAdapter;

/// Arguments for the "set" command
#[derive(Args, Debug, Clone)]
pub struct Print {
    #[arg(long, default_value = "default")]
    pub value: String,

    #[arg(long, default_value = "prefix")]
    pub prefix: String,
}

impl Print {
    pub async fn run(self) -> Result<()> {
        // Initialize Adapters
        let printer_adapter = PrinterMockAdapter::new(self.prefix);

        // Initialize services
        let printer_service = PrinterService::new(printer_adapter);

        // Execute service function
        printer_service.print(self.value).await
    }
}
