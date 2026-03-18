pub struct ErrorStatus {
    hadError: bool,
}

impl ErrorStatus {
    pub fn new() -> Self {
        Self { hadError: false }
    }

    pub fn yes_error(&mut self) {
        self.hadError = true;
    }
    pub fn no_error(&mut self) {
        self.hadError = false;
    }

    pub fn reset(&mut self) {
        self.hadError = false;
    }

    pub fn error(&self, line: u32, message: &str) {
        Self::report(line, "", message);
    }

    pub fn status(&self)->bool{
        return self.hadError;
    }

    fn report(line: u32, loc: &str, message: &str) {
        eprintln!("At line: {} {} Error: {}", line, loc, message);
    }
}
