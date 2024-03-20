use crate::{
    ast::{expression::IdentifierExp, Expression},
    token::TokenType,
};

use super::Parser;

impl Parser {
    pub(super) fn register_parse_functions(&self) {
        self.register_prefix(TokenType::Ident, Parser::parse_identifier);
    }

    // This method is used to parse the identifier expression.
    pub(super) fn parse_identifier(&self) -> Option<Box<dyn Expression>> {
        let cur_token = self.cur_token();
        let literal = cur_token.literal().to_string();
        let ident = IdentifierExp::new(cur_token, literal);

        Some(Box::new(ident))
    }
}
