use crate::ast::expression::IdentifierExp;
use crate::ast::{Expression, Node};
use crate::token::Token;

use super::Statement;

pub struct LetState {
    pub token: Token,
    pub name: IdentifierExp,
    pub value: Box<dyn Expression>,
}

impl LetState {
    pub fn new(token: Token, name: IdentifierExp, value: Box<dyn Expression>) -> LetState {
        LetState { token, name, value }
    }
}

impl Node for LetState {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str(&self.token.literal());
        out.push_str(" ");
        out.push_str(&self.name.to_string());
        out.push_str(" = ");
        out.push_str(&self.value.to_string());
        out.push_str(";");

        out
    }
}

impl Statement for LetState {
    fn statement_node(&self) {}
}
