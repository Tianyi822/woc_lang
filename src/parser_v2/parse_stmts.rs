use crate::{
    ast_v2::{expressions::IdentifierExp, statements::LetStatement},
    token::token::TokenType,
};

use super::parser::Parser;

impl Parser {
    pub(super) fn parse_let_stmt(&self) -> Option<LetStatement> {
        if !self.expect_peek(&TokenType::Ident) {
            return None;
        }

        // Move to Ident token
        let name = self.get_cur_token().literal().to_string();
        // Create IdentifierExp
        let ident_exp = IdentifierExp::new(name);

        // Move to Assign token
        if !self.expect_peek(&TokenType::Assignment) {
            return None;
        }
        // Skip Assign token
        self.next_token();

        // Parse the expression
        // TODO: because of the parsing logic of expression is not implemented yet, we will skip it for now.
        while !self.cur_token_is(&TokenType::Semicolon) {
            self.next_token();
        }

        Some(LetStatement::new(ident_exp, None))
    }

    pub(super) fn parse_return_stmt(&self) {
        todo!("parse_return_stmt")
    }
}
