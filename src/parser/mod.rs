use crate::ast::ast::Expression;
use crate::parser::parser::Parser;
use crate::token::precedence::*;

pub mod parse_exp_functions;
pub mod parse_stmt;
pub mod parser;

type PrefixParseFn = fn(&Parser) -> Option<Box<dyn Expression>>;
type InfixParseFn = fn(&Parser, Box<dyn Expression>) -> Option<Box<dyn Expression>>;
