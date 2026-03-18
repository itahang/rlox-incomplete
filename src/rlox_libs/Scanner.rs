#![allow(non_snake_case)]

use crate::rlox_libs::TokenTypes;
use crate::rlox_libs::{Literal::Literal, TokenTypes::TokensType};

use super::Token::Token;
use super::Utility::ERROR_STATUS as ERR;
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    line: usize,
    current: usize,

    _types: HashMap<String, TokensType>,
}
macro_rules! two_char_token {
    ($self:ident, $check:expr, $t_type1:expr, $t_type2:expr) => {{
        let token_type = if $self.matches($check) {
            $t_type1
        } else {
            $t_type2
        };
        $self.addSimpleToken(token_type);
    }};
}

impl Scanner {
    pub fn new(source: String) -> Self {
        let mut hm = HashMap::<String, TokensType>::new();

        hm.insert("and".to_string(), TokensType::AND);
        hm.insert("class".to_string(), TokensType::CLASS);
        hm.insert("else".to_string(), TokensType::ELSE);
        hm.insert("false".to_string(), TokensType::FALSE);
        hm.insert("for".to_string(), TokensType::FOR);
        hm.insert("fun".to_string(), TokensType::FUN);
        hm.insert("if".to_string(), TokensType::IF);
        hm.insert("nil".to_string(), TokensType::NIL);
        hm.insert("or".to_string(), TokensType::OR);
        hm.insert("print".to_string(), TokensType::PRINT);
        hm.insert("return".to_string(), TokensType::RETURN);
        hm.insert("super".to_string(), TokensType::SUPER);
        hm.insert("this".to_string(), TokensType::THIS);
        hm.insert("true".to_string(), TokensType::TRUE);
        hm.insert("var".to_string(), TokensType::VAR);
        hm.insert("while".to_string(), TokensType::WHILE);

        let tokens: Vec<Token> = Vec::new();
        Self {
            source,
            tokens,
            start: 0,
            line: 1,
            current: 0,
            _types: hm,
        }
    }

    pub fn addToken(&mut self, t_type: TokensType, literal: Literal) {
        let text = &self.source[self.start..self.current];

        self.tokens
            .push(Token::new(t_type, text.to_string(), literal, self.line));
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
            '(' => self.addSimpleToken(TokensType::LEFT_PAREN),
            ')' => self.addSimpleToken(TokensType::RIGHT_PAREN),
            '{' => self.addSimpleToken(TokensType::LEFT_BRACE),
            '}' => self.addSimpleToken(TokensType::RIGHT_BRACE),
            ',' => self.addSimpleToken(TokensType::COMMA),
            '.' => self.addSimpleToken(TokensType::DOT),
            '-' => self.addSimpleToken(TokensType::MINUS),
            '+' => self.addSimpleToken(TokensType::PLUS),
            ';' => self.addSimpleToken(TokensType::SEMICOLON),
            '*' => self.addSimpleToken(TokensType::STAR),
            '!' => {
                two_char_token!(self, '=', TokensType::BANG_EQUAL, TokensType::BANG);
            }
            '=' => {
                two_char_token!(self, '=', TokensType::EQUAL_EQUAL, TokensType::EQUAL);
            }
            '<' => {
                two_char_token!(self, '=', TokensType::LESS_EQUAL, TokensType::LESS);
            }
            '>' => {
                two_char_token!(self, '=', TokensType::GREATER_EQUAL, TokensType::GREATER);
            }
            '/' => {
                if self.matches('/') {
                    while self.peek() != '\n' && !self.isAtEnd() {
                        self.advance();
                    }
                } else {
                    self.addSimpleToken(TokensType::SLASH);
                }
            }
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string(), //  Not working
            'o' => {
                if self.matches('r') {
                    self.addSimpleToken(TokensType::OR);
                }
            }
            ch => {
                if Scanner::isDigit(ch) {
                    self.number(); // Not working
                    return;
                } else if ch.is_ascii_alphabetic() {
                    self.identifier();
                }

                ERR.lock().unwrap().yes_error();
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() {
            self.advance();
        }

         let text: String = self.source[self.start..self.current].chars().collect();

        let token_type = self
            ._types
            .get(&text)
            .copied()
            .unwrap_or(TokensType::IDENTIFIER);

        // Add the token (no mutation of the map needed)
        self.addSimpleToken(token_type);
    }

    fn isDigit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn number(&mut self) {
        while Scanner::isDigit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && Scanner::isDigit(self.peekNext()) {
            self.advance();

            while Scanner::isDigit(self.peek()) {
                self.advance();
            }
        }

        let num: String = self.source[self.start..self.current].chars().collect();
        let num = num.parse::<f64>().unwrap();

        self.addToken(TokensType::NUMBER, Literal::Number(num));
    }

    fn peekNext(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source.as_bytes()[self.current + 1] as char
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.isAtEnd() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }
        if self.isAtEnd() {
            ERR.lock().unwrap().error(self.line, "Unterminated string.");
            return;
        }

        self.advance();
        let line: String = self.source[self.start + 1..self.current - 1]
            .chars()
            .collect();
        self.addToken(TokensType::STRING, Literal::String(line));
    }

    fn matches(&mut self, expected: char) -> bool {
        if self.isAtEnd() {
            return false;
        }

        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.isAtEnd() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }
    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current] as char;
        self.current += 1;
        c
    }

    fn isAtEnd(&self) -> bool {
        self.current >= self.source.len()
    }
}
