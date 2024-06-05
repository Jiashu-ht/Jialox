#[derive(Debug)]
pub struct JialoxError {
    line: usize,
    message: String,
}

impl JialoxError {
    pub fn error(line: usize, message: String) -> JialoxError {
        JialoxError { line, message }
    }
    
    pub fn report(&self, loc: String) {
        eprintln!("[line {}] Error{}: {}", self.line, loc, self.message);
    }
}