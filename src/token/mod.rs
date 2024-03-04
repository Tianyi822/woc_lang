#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    PlusEqual,   // +=
    MinusEqual,  // -=
    StarEqual,   // *=
    SlashEqual,  // /=

    // Data Types
    Literal, // Literal
    Num,     // integer number: 1, 2, -3, 1_000 etc. Or float number: 1.0, 2.0, 3.0, etc.

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
            TokenType::Greater
            | TokenType::Less
            | TokenType::GreaterThanOrEqualTo
            | TokenType::LessThanOrEqualTo => 4,
            TokenType::Plus | TokenType::Minus => 5,
            TokenType::Star | TokenType::Slash | TokenType::Percent => 6,
            _ => 0,
        }
    }
}
