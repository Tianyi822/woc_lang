use std::fmt::Display;

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
    String,     // string: "hello world"

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

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let token = match self {
            TokenType::Comma => ",",
            TokenType::Dot => ".",
            TokenType::Semicolon => ";",
            TokenType::Colon => ":",
            TokenType::Assignment => "=",
            TokenType::LeftParen => "(",
            TokenType::RightParen => ")",
            TokenType::LeftBrace => "{",
            TokenType::RightBrace => "}",
            TokenType::LeftBracket => "[",
            TokenType::RightBracket => "]",
            TokenType::Quote => "\"",
            TokenType::SingleQuote => "'",
            TokenType::Not => "!",
            TokenType::Greater => ">",
            TokenType::Less => "<",
            TokenType::GreaterThanOrEqualTo => ">=",
            TokenType::LessThanOrEqualTo => "<=",
            TokenType::EqualTo => "==",
            TokenType::NotEqualTo => "!=",
            TokenType::And => "&&",
            TokenType::Or => "||",
            TokenType::BitAnd => "&",
            TokenType::BitOr => "|",
            TokenType::BitNot => "~",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Percent => "%",
            TokenType::PlusAssign => "+=",
            TokenType::MinusAssign => "-=",
            TokenType::AsteriskAssign => "*=",
            TokenType::SlashAssign => "/=",
            TokenType::Ident => "Identifier",
            TokenType::IntegerNum => "Integer",
            TokenType::FloatNum => "Float",
            TokenType::String => "String",
            TokenType::While => "while",
            TokenType::For => "for",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Break => "break",
            TokenType::Continue => "continue",
            TokenType::Let => "let",
            TokenType::Func => "function",
            TokenType::Return => "return",
            TokenType::Struct => "struct",
            TokenType::Enum => "enum",
            TokenType::None => "None",
            TokenType::True => "True",
            TokenType::False => "False",
            TokenType::Illegal => "Illegal",
            TokenType::Eof => "EOF",
        };
        write!(f, "{}", token)
    }
}