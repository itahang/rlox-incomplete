use super::Token::Token;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens: Vec<Token> = Vec::new();
        Self { source, tokens }
    }
    pub fn scanTokens(&self) -> &Vec<Token> {
        return &self.tokens;
    }
}
