use crate::ast::{Expression, Node};
use crate::token::Token;

// This struct is used to represent the identifier expression.
pub struct IdentifierExp {
    token: Token,
    value: String,
}

impl IdentifierExp {
    pub fn new(token: Token, value: String) -> IdentifierExp {
        IdentifierExp { token, value }
    }
}

impl Node for IdentifierExp {
    fn token_literal(&self) -> String {
        self.token.literal().to_string()
    }

    fn to_string(&self) -> String {
        self.value.clone()
    }
}

impl Expression for IdentifierExp {
    fn expression_node(&self) {}
}
