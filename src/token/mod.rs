#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenType {
    // Single Symbols
    Colon,        // :
    Minus,        // -
    Plus,         // +
    Slash,        // /
    Star,         // *
    Assignment,   // =
    Semicolon,    // ;
    Background,   // &
    GreaterThan,  // >
    LessThan,     // <
    Not,          // !
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Tilde,        // ~
    Quote,        // "
    SingleQuote,  // '

    // Combined Symbols
    DoubleMinus,          // --
    GreaterThanOrEqualTo, // >=
    LessThanOrEqualTo,    // <=
    EqualTo,              // ==
    NotEqualTo,           // !=
    And,                  // &&
    Or,                   // ||

    // Syntax Keywords
    While,    // while
    For,      // for
    If,       // if
    Else,     // else
    Break,    // break
    Continue, // continue
    Let,      // let
    Func,     // function
    Meth,     // method
    Return,   // return
    Struct,   // struct
    Enum,     // enum

    // Data Types
    Num,  // integer number: 1, 2, -3, 1_000 etc. Or float number: 1.0, 2.0, 3.0, etc.
    Str,  // string: "Hello, World!" or 'Hello, World!'
    Bool, // boolean: true or false
    None, // null: None

    // Literal
    Literal,

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

    pub fn priority(&self) -> i32 {
        match self.token_type {
            TokenType::Or => 1,
            TokenType::And => 2,
            TokenType::EqualTo | TokenType::NotEqualTo => 3,
            TokenType::GreaterThan
            | TokenType::LessThan
            | TokenType::GreaterThanOrEqualTo
            | TokenType::LessThanOrEqualTo => 4,
            TokenType::Plus | TokenType::Minus => 5,
            TokenType::Star | TokenType::Slash => 6,
            _ => 0,
        }
    }
}
