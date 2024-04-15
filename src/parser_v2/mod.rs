use crate::ast_v2::Expression;

use self::parser::Parser;

mod parse_stmts;
mod parse_exps;
pub mod parser;

// Parsing functions for prefix and infix expressions.
type PrefixParseFn = fn(&Parser) -> Option<Expression>;
type InfixParseFn = fn(&Parser, Expression) -> Option<Expression>;
