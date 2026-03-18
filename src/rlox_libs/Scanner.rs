#![allow(non_snake_case)]

use crate::rlox_libs::{Literal::Literal, TokenTypes::TokensType};

use super::Token::Token;
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    line: usize,
    current: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let tokens: Vec<Token> = Vec::new();
        Self {
            source,
            tokens,
            start: 0,
            line: 1,
            current: 0,
        }
    }

    pub fn addToken(&mut self, t_type: TokensType, literal: Literal) {
        let text: String = self.source[self.start..self.current].chars().collect();

        self.tokens
            .push(Token::new(t_type, text, literal, self.line));
    }

    pub fn addSimpleToken(&mut self, t_type: TokensType) {
        self.addToken(t_type, Literal::Nil);
    }

    pub fn scanTokens(&mut self) -> &Vec<Token> {
        while !(self.isAtEnd()) {
            self.start = self.current;
            self.scanToken();
        }
        self.tokens.push(Token::new(
            TokensType::EOF,
            String::from(""),
            Literal::Nil,
            self.line,
        ));
        return &self.tokens;
    }

    fn scanToken(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.addSimpleToken(TokensType::LEFT_BRACE),
            _ => {}
        }
    }

    fn advance(&self) -> char {
        return self.source.chars().nth(self.current).unwrap();
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len()
    }
}
