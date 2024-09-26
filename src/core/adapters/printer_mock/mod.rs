use crate::core::ports::PrinterPort;

pub struct PrinterMockAdapter {
    pub prefix: String,   
}

impl PrinterMockAdapter {
    pub fn new(prefix: String) -> Self {
        PrinterMockAdapter { prefix }
    }
}

impl PrinterPort for PrinterMockAdapter {
    fn print(&self, value: String) {
        println!("{}: {:?}", self.prefix, value);
    }
}