use anyhow::Result;

use crate::core::ports::PrinterPort;

pub struct PrinterService<T: PrinterPort> {
    printer: T,
}

impl<T: PrinterPort> PrinterService<T> {
    pub fn new(printer: T) -> Self {
        PrinterService { printer }
    }

    pub async fn print(&self, value: String) -> Result<()> {
        println!("Print called with: {:?}", value);

        self.printer.print(value);

        Ok(())
    }
}
