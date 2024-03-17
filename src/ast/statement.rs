use crate::ast::expression::IdentifierExp;
use crate::ast::{Expression, Node};
use crate::token::Token;

use super::Statement;

pub struct LetStatement {
    pub token: Token,
    pub name: IdentifierExp,
    pub value: Option<Box<dyn Expression>>,
}

// Let statement has a name, and the value is optional.
impl LetStatement {
    pub fn new(
        token: Token,
        name: IdentifierExp,
        value: Option<Box<dyn Expression>>,
    ) -> LetStatement {
        LetStatement { token, name, value }
    }
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        let mut out = String::new();

        out.push_str(&self.token.literal());
        out.push_str(" ");
        out.push_str(&self.name.to_string());

        if self.value.is_some() {
            out.push_str(" = ");
            out.push_str(&self.value.as_ref().map_or(String::new(), |v| v.to_string()));
        }

        out.push_str(";");

        out
    }
}

impl Statement for LetStatement {
    fn statement_node(&self) {}
}
