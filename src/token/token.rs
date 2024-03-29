use crate::token::precedence::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TokenType {
    // Single Symbols
    Comma,        // ,
    Dot,          // .
    Semicolon,    // ;
    Colon,        // :
    Assignment,   // =
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Quote,        // "
    SingleQuote,  // '

    // Logical calculation
    Not,                  // !
    Greater,              // >
    Less,                 // <
    GreaterThanOrEqualTo, // >=
    LessThanOrEqualTo,    // <=
    EqualTo,              // ==
    NotEqualTo,           // !=
    And,                  // &&
    Or,                   // ||

    // Bit calculation
    BitAnd, // &
    BitOr,  // |
    BitNot, // ~

    // Data calculate symbols
    Plus,           // +
    Minus,          // -
    Asterisk,       // *
    Slash,          // /
    Percent,        // %
    PlusAssign,     // +=
    MinusAssign,    // -=
    AsteriskAssign, // *=
    SlashAssign,    // /=

    // Data Types
    Ident,      // Identifier
    IntegerNum, // integer number: 1, 2, -3, 1_000 etc.
    FloatNum,   // float number: 1.0, 2.0, -3.0, 1_000.0 etc.

    // Syntax Keywords
    While,    // while
    For,      // for
    If,       // if
    Else,     // else
    Break,    // break
    Continue, // continue
    Let,      // let
    Func,     // function
    Return,   // return
    Struct,   // struct
    Enum,     // enum
    None,     // null: None
    True,     // true: True
    False,    // false: False

    // Illegal
    Illegal,

    // End of File
    Eof,
}

// This struct stores the token information that the lexer will analyze.
// And the parser will use the token to build the AST.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Token {
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }

    pub fn literal(&self) -> &str {
        &self.literal
    }

    pub fn token_type(&self) -> &TokenType {
        &self.token_type
    }

    pub fn is_eof(&self) -> bool {
        self.token_type == TokenType::Eof
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
            // others
            _ => LEVEL_0,
        }
    }
}
