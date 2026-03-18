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

    pub fn error(&mut self, line: usize, message: &str) {
        Self::report(line, "", message);
        self.hadError=true;
    }

    pub fn status(&self)->bool{
        return self.hadError;
    }

    fn report(line: usize, loc: &str, message: &str) {
        eprintln!("At line: {} {} Error: {}", line, loc, message);
        
    }
}
