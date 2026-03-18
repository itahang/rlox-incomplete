#![allow(non_snake_case)]

use std::fmt;

use crate::rlox_libs::TokenTypes;

use super::Literal::Literal;
use super::TokenTypes::TokensType;

pub struct Token {
    token_type: TokensType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

// public String toString() { return type + " " + lexeme + " " + literal; }
impl Token {
    pub fn new(t_type: TokensType, lexeme: String, literal: Literal, line: usize) -> Self {
        Self {
            token_type: t_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn addToken(&mut self,t_type:TokensType,literal:Literal){


    }

    pub fn toString(&self) -> String {
        return format!("{}", self);
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[cfg(test)]
mod test {
    use crate::rlox_libs::{Literal, Token::Token, TokenTypes::TokensType};

    #[test]
    fn testing_Token_formation() {
        let t = Token::new(TokensType::AND, String::from("&"), Literal::Literal::Nil, 0);
        let result = format!("{}", t);
        assert_eq!(result, "AND & nil");
    }
}
