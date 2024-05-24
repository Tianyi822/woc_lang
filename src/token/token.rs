use std::fmt::Display;

use crate::token::precedence::*;
use crate::token::types::TokenType;

// This struct stores the token information that the lexer will analyze.
// And the parser will use the token to build the AST.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    // It records the file path of the token.
    file_path: String,

    // It records the line number of the token.
    file_row_number: u32,

    // It records the column number of the token.
    file_col_number: u32,

    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Token {
        Token {
            file_path: String::new(),
            file_row_number: 0,
            file_col_number: 0,
            token_type,
            literal: literal.to_string(),
        }
    }

    pub fn is_eof(&self) -> bool {
        self.token_type == TokenType::Eof
    }

    pub fn is_semicolon(&self) -> bool {
        self.token_type == TokenType::Semicolon
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn precedence(&self) -> u32 {
        match self.token_type {
            // ||
            TokenType::Or => LEVEL_1,
            // &&
            TokenType::And => LEVEL_2,
            // ==, !=
            TokenType::EqualTo | TokenType::NotEqualTo => LEVEL_3,
            // >, <, >=, <=
            TokenType::Greater
            | TokenType::Less
            | TokenType::GreaterThanOrEqualTo
            | TokenType::LessThanOrEqualTo => LEVEL_4,
            // +, -
            TokenType::Plus | TokenType::Minus => LEVEL_5,
            // *, /, %
            TokenType::Asterisk | TokenType::Slash | TokenType::Percent => LEVEL_6,
            // func_name()
            TokenType::Func => LEVEL_7,
            // call
            TokenType::LeftParen => CALL,
            // Array index
            TokenType::LeftBracket => LEVEL_10,
            // others
            _ => LEVEL_0,
        }
    }
}
